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
import sys

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
        # print("Input ids: ", input_ids)
        # print("Target ids: ", target_ids)
        # print(end_loc, trg_len, prev_end_loc)

        with torch.no_grad():
            outputs = model(input_ids, target_ids)

            # loss is calculated using CrossEntropyLoss which averages over valid labels
            # N.B. the model only calculates loss over trg_len - 1 labels, because it internally shifts the labels
            # to the left by 1.
            neg_log_likelihood = outputs[1]  # Get the loss from the outputs
            # print("outputs[0]: ", outputs[0])
            # print("Negative log likelihood: ", neg_log_likelihood.item())

        # Accumulate the total negative log-likelihood and the total number of tokens
        num_valid_tokens = (target_ids != -1).sum().item()  # number of valid tokens in target_ids
        batch_size = target_ids.size(0)
        num_loss_tokens = num_valid_tokens - batch_size  # subtract batch_size due to internal label shift
        nll_sum += neg_log_likelihood * num_loss_tokens
        n_tokens += num_loss_tokens
        # print("NLL: ", neg_log_likelihood.item())
        # print("NLL sum: ", nll_sum)
        # print("Number of tokens: ", n_tokens)
        # print("Previous end location: ", prev_end_loc)
        # print("End location: ", end_loc)
        # print("Batch size: ", batch_size)
        # print("Number of loss tokens: ", num_loss_tokens)
        # print("Number of valid tokens: ", num_valid_tokens)

        prev_end_loc = end_loc
        if end_loc == seq_len:
            print("End location reached the sequence length. ", end_loc, seq_len)
            break

    avg_nll = nll_sum / n_tokens  # average negative log-likelihood per token
    print("NLL sum: ", nll_sum)
    print("Number of tokens: ", n_tokens)
    print("Average NLL: ", avg_nll.item())
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

def calculate_ppl_spmuni_tamil():
    spm_model_plain='vocab_models/sentencepiece/spmuni_tamil_12k.model'
    spm_model_pretok='vocab_models/sentencepiece/spmuni_tamil_pretok.model'
    spm_tokenizer_plain =  spm.SentencePieceProcessor(model_file=spm_model_plain)
    spm_tokenizer_pretok =  spm.SentencePieceProcessor(model_file=spm_model_pretok)
    brahmi_model = 'tamil.json'
    brahmi_tokenizer = brahmi_script.Tokenizer("tamil", brahmi_model)
    files = ['/home/rakshika/Downloads/archive/valid/valid/AA_wiki_test_1.txt',
            '/home/rakshika/Downloads/archive/valid/valid/AA_wiki_test_2.txt',]
    for file in files:
        print("File: ", file)
        visited = {}
        training_size = 1000000
        out_dir_plain = 'out-tamwiki-spuni-12k'
        out_dir_pretok = 'out-tamwiki-spuni-pretok'
        model_plain = load_model(out_dir_plain)
        model_pretok = load_model(out_dir_pretok)
        encoded = encode_input(brahmi_tokenizer, spm_tokenizer_plain, file, training_size, visited).unsqueeze(0)
        calc_ppl_score(model_plain, encoded)
        # encoded = encode_input(brahmi_tokenizer, spm_tokenizer_pretok, file, training_size, visited, pretok=True).unsqueeze(0)
        # calc_ppl_score(model_pretok, encoded)

def calculate_ppl_spmuni_telugu():
    brahmi_model = 'telugu.json'
    # spm_model_plain='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/uni-8000.model'
    # spm_tokenizer_plain =  spm.SentencePieceProcessor(model_file=spm_model_plain)
    spm_model_pretok='vocab_models/sentencepiece/telbpe.model'
    spm_tokenizer_pretok =  spm.SentencePieceProcessor(model_file=spm_model_pretok)
    brahmi_tokenizer = brahmi_script.Tokenizer("telugu", brahmi_model)
    files = ['clean-articles.txt',]
    for file in files:
        print("File: ", file)
        visited = {}
        training_size = 1100000
        out_dir_pretok = 'out-telwiki-spbpe'
        model_pretok = load_model(out_dir_pretok)
        encoded = encode_input(brahmi_tokenizer, spm_tokenizer_pretok, file, training_size, visited, pretok=False).unsqueeze(0)
        calc_ppl_score(model_pretok, encoded)

def encode_input(brahmi_tokenizer, spm_tokenizer, input, target_size, visited, pretok=True):
    tensor = torch.tensor([], dtype=torch.long)
    if os.path.isfile(input):
        if pretok:
            tensor = append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, input, tensor)
        else:
            tensor = append_to_torch_spuni(spm_tokenizer, input, tensor)
        return tensor
    print("Entering directory: ", input)
    print("Total files: ", len(os.listdir(input)))
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
            if pretok:
                new_tensor = append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, file, tensor)
            else:
                new_tensor = append_to_torch_spuni(spm_tokenizer, file, tensor)
            # print("New tensor size: ", new_tensor.size())
            if len(new_tensor) > target_size:
                # train_ids = new_tensor.numpy(force=True)
                # train_ids.tofile(os.path.join(os.path.dirname(__file__), name))
                print("File count: ", file_count)
                return tensor
            else:
                tensor = new_tensor
    print("File count: ", file_count)
    return tensor

def append_to_torch_spuni(spm_tokenizer, file_path, tensor):
    text = open(file_path, 'r', encoding='utf-8').read()

    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = spm_tokenizer.encode(text, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.long))) 

def append_to_torch_spuni_pretok(brahmi_tokenizer, spm_tokenizer, file_path, tensor):
    text = open(file_path, 'r', encoding='utf-8').read()

    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = brahmi_tokenizer.transform_encode(text)
    encoded = spm_tokenizer.encode(encoded, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.long)))

if __name__ == "__main__":
    # calculate_ppl_spmuni_tamil()
    calculate_ppl_spmuni_telugu()
