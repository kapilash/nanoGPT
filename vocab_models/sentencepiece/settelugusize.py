import sys
import shutil as os

target_bpe_model = 'telbpe.model'
target_bpe_vocab = 'telbpe.vocab'

target_uni_model = 'telunigram.model'
target_uni_vocab = 'telunigram.vocab'

target_bpet_model = 'telbpet.model'
target_bpet_vocab = 'telbpet.vocab'

target_unit_model = 'telunigramt.model'
target_unit_vocab = 'telunigramt.vocab'

def copy_bpe_files(size):
    model_format = 'bpe-{}000.model'
    vocab_format = 'bpe-{}000.vocab'
    model_file = model_format.format(size)
    vocab_file = vocab_format.format(size)
    os.copyfile(model_file, target_bpe_model)
    os.copyfile(vocab_file, target_bpe_vocab)
    return model_file, vocab_file

def copy_unigram_files(size):
    model_format = 'uni-{}000.model'
    vocab_format = 'uni-{}000.vocab'
    model_file = model_format.format(size)
    vocab_file = vocab_format.format(size)
    os.copyfile(model_file, target_uni_model)
    os.copyfile(vocab_file, target_uni_vocab)
    return model_file, vocab_file

def copy_bpet_files(size):
    model_format = 'pretok-bpe-{}000.model'
    vocab_format = 'pretok-bpe-{}000.vocab'
    model_file = model_format.format(size)
    vocab_file = vocab_format.format(size)
    os.copyfile(model_file, target_bpet_model)
    os.copyfile(vocab_file, target_bpet_vocab)
    return model_file, vocab_file

def copy_unit_files(size):
    model_format = 'pretok-uni-{}000.model'
    vocab_format = 'pretok-uni-{}000.vocab'
    model_file = model_format.format(size)
    vocab_file = vocab_format.format(size)
    os.copyfile(model_file, target_unit_model)
    os.copyfile(vocab_file, target_unit_vocab)
    return model_file, vocab_file


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python settelugusize.py <size>")
        sys.exit(1)
    args = sys.argv[1:]
    size = int(args[0])
    if size < 8 or size > 12:
        print("Size must be between 8 and 12")
        sys.exit(1)
    copy_bpe_files(size)
    copy_unigram_files(size)
    print("Copied files for size: ", size)

