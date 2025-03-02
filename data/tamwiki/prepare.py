
import sentencepiece as spm
import os
import pickle
import requests
import numpy as np
import sys
import torch
import re
import brahmi_script

# encode a text and append the result to a torch tensor
def append_to_torch_pretok(brahmi_tokenizer, spm_tokenizer, file_path, tensor):
    file_size = os.path.getsize(file_path)
    text = open(file_path, 'r', encoding='utf-8').read()
    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = brahmi_tokenizer.transform_encode(text)
    encoded = spm_tokenizer.encode(encoded, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.int16))) 

def append_to_torch(tokenizer, file_path, tensor):
    file_size = os.path.getsize(file_path)
    text = open(file_path, 'r', encoding='utf-8').read()
    # Remove HTML tags and the text within them
    text = re.sub(r'<[^>]*>', '', text)
    encoded = tokenizer.encode(text, out_type=int)
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.int16))) 

# recursively go through a directory and encode files till the tensor size reaches target_size
def wiki_encdec_dir(brahmi_tokenizer, spm_tokenizer, directory, tensor, target_size, visited, name, pretok):
    print("Entering directory: ", directory)
    file_count = 0
    for f in os.listdir(directory):
        file = os.path.join(directory, f)
        if os.path.isfile(file):
            if visited.get(file):
                print("skipping :", file)
                continue
            visited[file] = True
            file_count += 1
            # print("file: ", file)
            if pretok:
                new_tensor = append_to_torch_pretok(brahmi_tokenizer, spm_tokenizer, file, tensor)
            else:
                new_tensor = append_to_torch(spm_tokenizer, file, tensor)
            if len(new_tensor) > target_size:
                train_ids = new_tensor.numpy(force=True)
                train_ids.tofile(os.path.join(os.path.dirname(__file__), name))
                print("File count: ", file_count)
                return 
            else:
                tensor = new_tensor
        if os.path.isdir(file):
            tensor, file_count = wiki_encdec_dir(brahmi_tokenizer, spm_tokenizer, file)
    print("File count: ", file_count)
    return 

if __name__ == "__main__":
    args = sys.argv[1:]
    directory = args[0]
    spm_model = args[1]
    pretok = args[2].lower() == 'true'
    print(directory, spm_model, pretok)
    spm_tokenizer = spm.SentencePieceProcessor(model_file=spm_model)
    brahmi_tokenizer = brahmi_script.Tokenizer("tamil", "../../tamil.json")
    tensor = torch.tensor([], dtype=torch.int16)
    visited = {}
    training_size = 11000000
    wiki_encdec_dir(brahmi_tokenizer, spm_tokenizer, directory, tensor, training_size, visited, "train.bin", pretok)
    print("Training data saved to train.bin")
    tensor = torch.tensor([], dtype=torch.int16)
    validation_size = 1100000
    wiki_encdec_dir(brahmi_tokenizer, spm_tokenizer, directory, tensor, validation_size, visited, "val.bin", pretok)
    meta = {
        'vocab_size': len(spm_tokenizer),
        'itos': "sentencepiece.Tokenizer",
        'stoi': "sentencepiece.Tokenizer",
    }
    with open(os.path.join(os.path.dirname(__file__), 'meta.pkl'), 'wb') as f:
        pickle.dump(meta, f)

    print("Done")
