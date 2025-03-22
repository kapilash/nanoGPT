"""
Prepare the Telugu dataset for character level language modeling.
So instead of encoding with GPT-2 BPE tokens, we use sentence piece BPE tokenizer that has been preprocessed by brahmi_script's tokenizer. The result has a vocabulary size of 8000.
Will save train.bin, val.bin containing the ids, and meta.pkl containing the
info related to the vocabulary size.
"""
import sentencepiece as spm
import brahmi_script
import os
import pickle
import requests
import numpy as np
import sys
import torch

# encode a text and append the result to a torch tensor
def append_to_torch(tokenizer, bt, file_path, tensor):
    file_size = os.path.getsize(file_path)
    text = open(file_path, 'r').read()
    transformed = bt.transform_encode(text)
    encoded = tokenizer.encode(transformed, out_type=int)
    for i in range(len(encoded)):
        if encoded[i] == 0:
            print("Error in encoding: ", file_path)
            #print("Encoded text: ", transformed)
            #print("File: ", file_path)
            #print("Text: ", text)
            return tensor
    return torch.cat((tensor, torch.tensor(encoded, dtype=torch.int16))) 

# recursively go through a directory and encode files till the tensor size reaches target_size
def wiki_encdec_dir(tokenizer, bt, directory, tensor, target_size, visited, name):
    print("Entering directory: ", directory)
    for f in os.listdir(directory):
        file = os.path.join(directory, f)
        if os.path.isfile(file):
            if visited.get(file):
                print("skipping :", file)
                continue
            visited[file] = True
            #print("file: ", file)
            new_tensor = append_to_torch(tokenizer, bt, file, tensor)
            if len(new_tensor) > target_size:
                train_ids = new_tensor.numpy(force=True)
                train_ids.tofile(os.path.join(os.path.dirname(__file__), name))
                return 
            else:
                tensor = new_tensor
        elif os.path.isdir(file):
            tensor, file_count = wiki_encdec_dir(tokenizer, bt, file, tensor, target_size, visited, name)
    return 

if __name__ == "__main__":
    args = sys.argv[1:]
    directory = args[0]
    tokenizer = spm.SentencePieceProcessor(model_file='../../vocab_models/sentencepiece/telbpet.model')
    bt = brahmi_script.Tokenizer("telugu", "smf.json")
    tensor = torch.tensor([], dtype=torch.int16)
    visited = {}
    training_size = 1100000
    wiki_encdec_dir(tokenizer, bt, directory, tensor, training_size, visited, "train.bin")
    print("Training data saved to train.bin")
    tensor = torch.tensor([], dtype=torch.int16)
    validation_size = 110000
    wiki_encdec_dir(tokenizer, bt, directory, tensor, validation_size, visited, "val.bin")
    meta = {
        'vocab_size': len(tokenizer),
        'itos': "sentencepiece.Tokenizer",
        'stoi': "sentencepiece.Tokenizer",
    }
    print(meta)
    with open(os.path.join(os.path.dirname(__file__), 'meta.pkl'), 'wb') as f:
        pickle.dump(meta, f)

    print("Done")
