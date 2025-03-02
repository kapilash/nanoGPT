from transformers import GPT2LMHeadModel, GPT2TokenizerFast
from accelerate.test_utils.testing import get_backend
from datasets import load_dataset
import os
import torch
from tqdm import tqdm
from model import GPTConfig, GPT
import sentencepiece as spm
import re
import brahmi_script

device, _, _ = get_backend() # automatically detects the underlying device type (CUDA, CPU, XPU, MPS, etc.)

def calc_ppl_score(model, encodings):
    max_length = 256
    stride = 128
    seq_len = encodings.size(1)
    print("Sequence length: ", seq_len)

    nll_sum = 0.0
    n_tokens = 0
    prev_end_loc = 0
    for begin_loc in tqdm(range(0, seq_len, stride)):
        end_loc = min(begin_loc + max_length, seq_len)
        trg_len = end_loc - prev_end_loc  # may be different from stride on last loop
        input_ids = encodings[:, begin_loc:end_loc].to(device)  # Add batch dimension
        target_ids = input_ids.clone()
        target_ids = target_ids[:, 1:]
        target_ids[:, :-trg_len] = -1
        input_ids = input_ids[:, :-1]

        with torch.no_grad():
            outputs = model(input_ids, target_ids)

            # loss is calculated using CrossEntropyLoss which averages over valid labels
            # N.B. the model only calculates loss over trg_len - 1 labels, because it internally shifts the labels
            # to the left by 1.
            neg_log_likelihood = outputs[1]  # Get the loss from the outputs

        # Accumulate the total negative log-likelihood and the total number of tokens
        num_valid_tokens = (target_ids != -1).sum().item()  # number of valid tokens in target_ids
        batch_size = target_ids.size(0)
        num_loss_tokens = num_valid_tokens - batch_size  # subtract batch_size due to internal label shift
        nll_sum += neg_log_likelihood * num_loss_tokens
        n_tokens += num_loss_tokens

        prev_end_loc = end_loc
        if end_loc == seq_len:
            break

    avg_nll = nll_sum / n_tokens
    ppl = torch.exp(avg_nll)
    print(f"Perplexity: {ppl.item()}")

def load_model(out_dir):
    ckpt_path = os.path.join(out_dir, 'ckpt.pt')
    checkpoint = torch.load(ckpt_path, map_location=device)
    gptconf = GPTConfig(**checkpoint['model_args'])
    model = GPT(gptconf)
    state_dict = checkpoint['model']
    unwanted_prefix = '_orig_mod.'
    for k,v in list(state_dict.items()):
        if k.startswith(unwanted_prefix):
            state_dict[k[len(unwanted_prefix):]] = state_dict.pop(k)
    model.load_state_dict(state_dict)
    model.eval()
    model.to(device)
    return model

def calculate_ppl_spmuni_tamil(spm_model, model_out_dir, test_data, pretok=False):
    spm_tokenizer =  spm.SentencePieceProcessor(model_file=spm_model)
    brahmi_model = 'tamil.json'
    brahmi_tokenizer = brahmi_script.Tokenizer("tamil", brahmi_model)

    print("Data: ", test_data)
    visited = {}
    training_size = 1000000
    model_plain = load_model(model_out_dir)
    encoded = encode_input(brahmi_tokenizer, spm_tokenizer, test_data, training_size, visited, pretok).unsqueeze(0)
    calc_ppl_score(model_plain, encoded)

def calculate_ppl_spmuni_telugu():
    brahmi_model = 'telugu.json'
    spm_model_plain='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/telunigram.model'
    spm_tokenizer_plain =  spm.SentencePieceProcessor(model_file=spm_model_plain)
    spm_model_pretok='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/telunigramt.model'
    spm_tokenizer_pretok =  spm.SentencePieceProcessor(model_file=spm_model_pretok)
    brahmi_tokenizer = brahmi_script.Tokenizer("telugu", brahmi_model)
    files = ['/home/rakshika/Downloads/telugu_data/clean-articles.txt',]
    for file in files:
        print("File: ", file)
        visited = {}
        training_size = 1100000
        out_dir_plain = 'out-telwiki-spuni'
        model_plain = load_model(out_dir_plain)
        encoded = encode_input(brahmi_tokenizer, spm_tokenizer_plain, file, training_size, visited).unsqueeze(0)
        calc_ppl_score(model_plain, encoded)

        out_dir_pretok = 'out-telwiki-spunit'
        model_pretok = load_model(out_dir_pretok)
        encoded = encode_input(brahmi_tokenizer, spm_tokenizer_pretok, file, training_size, visited, pretok=True).unsqueeze(0)
        calc_ppl_score(model_pretok, encoded)

def encode_input(brahmi_tokenizer, spm_tokenizer, input, target_size, visited, pretok=False):
    tensor = torch.tensor([], dtype=torch.long)
    if os.path.isfile(input):
        file_extension = os.path.splitext(input)[1]
        file_text = open(input, 'r', encoding='utf-8').read()
        if file_extension == '.txt':
            if pretok:
                tensor = append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, file_text, tensor)
            else:
                tensor = append_to_torch_spuni(spm_tokenizer, file_text, tensor)
        else:
            # File is a csv
            # Read the CSV file and extract the text column
            import pandas as pd
            df = pd.read_csv(input)
            df = df.head(2000)
            if 'news_article' in df.columns:
                text_column = df['news_article']
                for text in text_column:
                    if pretok:
                        tensor = append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, text, tensor)
                    else:
                        tensor = append_to_torch_spuni(spm_tokenizer, text, tensor)
        return tensor

    # Input is a directory
    # print("Entering directory: ", input)
    # print("Total files: ", len(os.listdir(input)))
    file_count = 0
    for f in os.listdir(input):
        file = os.path.join(input, f)
        if os.path.isfile(file):
            if visited.get(file):
                print("skipping :", file)
                continue
            file_count += 1
            visited[file] = True
            #print("file: ", file)
            file_text = open(file, 'r', encoding='utf-8').read()
            if pretok:
                new_tensor = append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, file_text, tensor)
            else:
                new_tensor = append_to_torch_spuni(spm_tokenizer, file_text, tensor)
            # print("New tensor size: ", new_tensor.size())
            if len(new_tensor) > target_size:
                return tensor
            else:
                tensor = new_tensor
    # print("File count: ", file_count)
    return tensor

def append_to_torch_spuni(spm_tokenizer, text, tensor):
    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = spm_tokenizer.encode(text, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.long))) 

def append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, text, tensor):
    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = brahmi_tokenizer.transform_encode(text)
    encoded = spm_tokenizer.encode(encoded, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.long)))

if __name__ == "__main__":
    print("Tamil unigram plain 8k tested on Tamil news dataset")
    calculate_ppl_spmuni_tamil('vocab_models/sentencepiece/tamil_models/spmuni_tamil_8k.model', 'out-tamwiki-spuni-8k', "/home/rakshika/Downloads/tamilmurasu_archive/tamilmurasu_dataset.csv")
    print("Tamil BPE plain 8k tested on Tamil news dataset")
    calculate_ppl_spmuni_tamil('vocab_models/sentencepiece/tamil_models/spmbpe_tamil_8k.model', 'out-tamwiki-spbpe-8k', "/home/rakshika/Downloads/tamilmurasu_archive/tamilmurasu_dataset.csv")
    print("Tamil unigram pretok 8k tested on Tamil news dataset")
    calculate_ppl_spmuni_tamil('vocab_models/sentencepiece/tamil_models/spmuni_tamil_pretok_8k.model', 'out-tamwiki-spuni-pretok-8k', "/home/rakshika/Downloads/tamilmurasu_archive/tamilmurasu_dataset.csv", pretok=True)
    print("Tamil BPE pretok 8k tested on Tamil news dataset")
    calculate_ppl_spmuni_tamil('vocab_models/sentencepiece/tamil_models/spmbpe_tamil_pretok_8k.model', 'out-tamwiki-spbpe-pretok-8k', "/home/rakshika/Downloads/tamilmurasu_archive/tamilmurasu_dataset.csv", pretok=True)
    