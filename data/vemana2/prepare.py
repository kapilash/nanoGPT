"""
Encode Poetry from Vemana Satakam into characters using brahmi_lipi tokenizer
"""
import tiktoken
import os
import pickle
import requests
import numpy as np
import sys
import torch

# encode a text and return the tensor
def encode_text(tokenizer, file_path):
    with open(file_path, 'r',encoding="utf8") as fd:
        file_text = fd.read()
        encoded = tokenizer.encode(file_text)
    return torch.tensor(encoded, dtype=torch.int32) 


if __name__ == "__main__":
    args = sys.argv[1:]
    training_file = args[0]
    tokenizer = tiktoken.get_encoding("cl100k_base")
    #tokenizer = tiktoken.get_encoding("gpt2")
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
        'vocab_size': tokenizer.max_token_value + 1,
        'itos': "brahmi_lipi.TeluguTokenizer",
        'stoi': "brahmi_lipi.TeluguTokenizer",
    }
    with open(os.path.join(os.path.dirname(__file__), 'meta.pkl'), 'wb') as f:
        pickle.dump(meta, f)

    print("Done")
