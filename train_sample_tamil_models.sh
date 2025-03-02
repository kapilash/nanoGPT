#!/bin/bash
spm_uni_model='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/tamil_models/spmuni_tamil_8k.model'
spm_bpe_model='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/tamil_models/spmbpe_tamil_8k.model'
spm_uni_pretok_model='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/tamil_models/spmuni_tamil_pretok_8k.model'
spm_bpe_pretok_model='/home/rakshika/src/kapilash/nanoGPT/vocab_models/sentencepiece/tamil_models/spmbpe_tamil_pretok_8k.model'
train_data='/home/rakshika/Downloads/archive/train/train'
config_file='config/train_tamwiki.py'

prepare() {
    spm_model=$1
    pretok=$2
    echo "Preparing data with model: $spm_model, pretok: $pretok"
    cd data/tamwiki
    rm 'meta.pkl' 'train.bin' 'val.bin'
    python prepare.py $train_data $spm_model $pretok
    cd ../..
}

train() {
    config_file=$1
    out_dir=$2
    rm -rf $out_dir
    python train.py $config_file --out_dir=$out_dir --dataset=tamwiki
}

sample() {
    out_dir=$1
    spm_model=$2
    pretok=$3
    python sample_tamil.py --out_dir=$out_dir --spm_model=$spm_model --pretok=$pretok
}

out_dirs=('out-tamwiki-spuni-8k' 'out-tamwiki-spbpe-8k' 'out-tamwiki-spuni-pretok-8k' 'out-tamwiki-spbpe-pretok-8k')
spm_models=("$spm_uni_model" "$spm_bpe_model" "$spm_uni_pretok_model" "$spm_bpe_pretok_model")

for i in "${!out_dirs[@]}"; do
    spm_model="${spm_models[$i]}"
    if [[ "$spm_model" == *pretok* ]]; then
        pretok=True
    else
        pretok=False
    fi
    echo "Processing model: $spm_model, pretok: $pretok"
    prepare "$spm_model" $pretok
    train $config_file "${out_dirs[$i]}"
    sample "${out_dirs[$i]}" "${spm_models[$i]}" $pretok
done