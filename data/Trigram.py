import numpy as np
import sentencepiece as spm
import tiktoken
import torch
import brahmi_script
import sys

class TokenPair:
    def __init__(self, token1, token2):
        self.token1 = token1
        self.token2 = token2

    def __repr__(self):
        return f"TokenPair({self.token1}, {self.token2})"

    def __eq__(self, other):
        return (self.token1 == other.token1) and (self.token2 == other.token2)

    def __hash__(self):
        return hash((self.token1, self.token2))


class TrigramModel:
    def __init__(self, tokenizer_name, vocabulary_size=8000):
        self.tokenizer_name = tokenizer_name
        self.vocabulary_size = vocabulary_size
        self.trigrams = {}
        self.probabilities = {}

    def add_token(self, token1, token2, token3):
        token_pair = TokenPair(token1, token2)
        if token_pair not in self.trigrams:
            self.trigrams[token_pair] = np.ones(self.vocabulary_size)
        self.trigrams[token_pair][token3] += 1

    def compute_probability(self):
        probabilities = {}
        for token_pair, counts in self.trigrams.items():
            total_count = np.sum(counts)
            probabilities[token_pair] = counts  / total_count
            #print("probability of ", token_pair, " is ", probabilities[token_pair])
        self.probabilities = probabilities

    def get_probability(self, token1, token2, token3):
        token_pair = TokenPair(token1, token2)
        if token_pair not in self.probabilities:
            #print("token pair not found ", token_pair)
            return None
        probabilities = self.probabilities[token_pair]
        #print("probabilities of ", token_pair, " are ", probabilities, " and ", probabilities[token3])
        if token3 >= len(probabilities):
            print("token3 is out of range ", token3, " for ", probabilities)
            return None
        return probabilities[token3] 

class Perplexity:
    def __init__(self, trigram_model, vocabulary_size=8000):
        self.trigram_model = trigram_model
        self.vocabulary_size = vocabulary_size
        self.probability = 1

    def add_token(self, token1, token2, token3):
        probability = self.trigram_model.get_probability(token1, token2, token3)
        if probability is not None:
            if probability < 0.00001:
                print("probability is ", probability, " for [ ", token1, token2, token3, "]")
            self.probability *= probability
        else:
            self.probability *= 1 / self.vocabulary_size
        if self.probability < 0.00001:
            print("probability is ", self.probability, " for [ ", token1, token2, token3, "]")
        #print("probability ", self.probability)
        return self.probability

    def calculate_perplexity(self):
        if self.probability == 0:
            return float('inf')
        return 1 / (self.probability ** (1 / 3))


if __name__ == "__main__":
    tokenizer_name = "sentencepiece"
    vocabulary_size = 8000
    train_file = None
    test_file = None
    args = sys.argv
    if len(args) > 1:
        vocabulary_size = int(args[1])
    if len(args) > 2:
        train_file = args[2]
    if len(args) > 3:
        test_file = args[3]
    if len(args) > 4:
        tokenizer_name = args[4]
    if train_file is None:
        sys.exit("Please provide a training file.")
    if test_file is None:
        sys.exit("Please provide a test file.")

    # load the training data
    data = np.memmap(train_file, dtype='uint16', mode='r')
    if len(data) < 3:
        sys.exit("Training data is too short.")

    # train the model
    model = TrigramModel(tokenizer_name, vocabulary_size)
    token1 = data[0]
    token2 = data[1]
    print("Loaded training data ", len(data), " tokens")
    for i in range(2, len(data)):
        token3 = data[i]
        model.add_token(token1, token2, token3)
        token1 = token2
        token2 = token3

    model.compute_probability()
    # load the test data
    test_data = np.memmap(test_file, dtype='uint16', mode='r')
    if len(test_data) < 3:
        sys.exit("Test data is too short.")
    print("Loaded test data ", len(test_data), " tokens")
    perplexity = Perplexity(model, vocabulary_size)
    token1 = test_data[0]
    token2 = test_data[1]
    for i in range(2, len(test_data)):
        token3 = test_data[i]
        perplexity.add_token(token1, token2, token3)
        token1 = token2
        token2 = token3
    perplexity_score = perplexity.calculate_perplexity()
    print(f"Perplexity: {perplexity_score} for {tokenizer_name} with vocabulary size {vocabulary_size}")
    

