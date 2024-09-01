"""
Encode Poetry from Gulzaar and Saahil Ludhianvi into characters using brahmi_script tokenizer
"""
import brahmi_script
import os
import pickle
import requests
import numpy as np
import sys
import torch

# encode a text and return the tensor
def encode_text(tokenizer, file_path):
    file_size = os.path.getsize(file_path)
    encoded = tokenizer.encode_file(file_path)
    for e in encoded :
        if e >= 977 :
            print("Found ", e)
    return torch.tensor(encoded, dtype=torch.int16) 


if __name__ == "__main__":
    args = sys.argv[1:]
    training_file = args[0]
    tokenizer = brahmi_script.Tokenizer("devnagari", "hindivocab.json")
    train_tensor = encode_text(tokenizer, training_file)
    train_ids = train_tensor.numpy(force=True)
    train_ids.tofile(os.path.join(os.path.dirname(__file__), "train.bin"))
    print("Training data saved to train.bin")
    validation_file = args[1]
    validation_tensor = encode_text(tokenizer, validation_file)
    validation_ids = validation_tensor.numpy(force=True)
    validation_ids.tofile(os.path.join(os.path.dirname(__file__), "val.bin"))
    print("Validation data saved to val.bin")

    meta = {
        'vocab_size': 977,
        'itos': "brahmi_script.Tokenizer",
        'stoi': "brahmi_script.Tokenizer",
    }
    with open(os.path.join(os.path.dirname(__file__), 'meta.pkl'), 'wb') as f:
        pickle.dump(meta, f)

    print("Done")
