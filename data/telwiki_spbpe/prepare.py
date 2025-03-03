"""
Prepare the Telugu dataset for character level language modeling.
So instead of encoding with GPT-2 BPE tokens, we use sentence piece bpe tokenizer. This has a vocabulary size of 15651.
Will save train.bin, val.bin containing the ids, and meta.pkl containing the
info related to the vocabulary size.
"""
import sentencepiece as spm
import os
import pickle
import requests
import numpy as np
import sys
import torch

# encode a text and append the result to a torch tensor
def append_to_torch(tokenizer, file_path, tensor):
    text = open(file_path, 'r').read()
    with open(file_path, 'r') as fd:
        file_text = fd.read()
        encoded = tokenizer.encode(file_text, out_type=int)
        return torch.cat((tensor, torch.tensor(encoded, dtype=torch.int16))) 

# recursively go through a directory and encode files till the tensor size reaches target_size
def wiki_encdec_dir(tokenizer, directory, tensor, target_size, visited, name):
    print("Entering directory: ", directory)
    for f in os.listdir(directory):
        file = os.path.join(directory, f)
        if os.path.isfile(file):
            if visited.get(file):
                print("skipping :", file)
                continue
            visited[file] = True
            #print("file: ", file)
            new_tensor = append_to_torch(tokenizer, file, tensor)
            if len(new_tensor) > target_size:
                train_ids = new_tensor.numpy(force=True)
                print("new tensor size: ", new_tensor.size())
                print("new tensor", new_tensor)
                train_ids.tofile(os.path.join(os.path.dirname(__file__), name))
                return 
            else:
                tensor = new_tensor
        if os.path.isdir(file):
            tensor, file_count = wiki_encdec_dir(tokenizer, file)
    return 

if __name__ == "__main__":
    args = sys.argv[1:]
    directory = args[0]
    tokenizer = spm.SentencePieceProcessor(model_file='../../vocab_models/sentencepiece/telbpe.model')
    tensor = torch.tensor([], dtype=torch.int16)
    visited = {}
    training_size = 1100000
    wiki_encdec_dir(tokenizer, directory, tensor, training_size, visited, "train.bin")
    print("Training data saved to train.bin")
    tensor = torch.tensor([], dtype=torch.int16)
    validation_size = 110000
    wiki_encdec_dir(tokenizer, directory, tensor, validation_size, visited, "val.bin")
    
    meta = {
        'vocab_size': len(tokenizer),
        'itos': "sentencepiece.Tokenizer",
        'stoi': "sentencepiece.Tokenizer",
    }
    with open(os.path.join(os.path.dirname(__file__), 'meta.pkl'), 'wb') as f:
        pickle.dump(meta, f)

    print("Done")
