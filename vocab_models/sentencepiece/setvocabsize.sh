MODELSIZE=$1
FILESUFFIX="{$1}00"

if [ -z "$MODELSIZE" ]; then
  echo "Usage: $0 <vocab_size>"
  exit 1
fi
echo "Setting vocab size to $MODELSIZE"

if [ $MODELSIZE -eq 8 ] then
    echo "Vocab size is 8, setting to 8k"
    MODELSIZE=8k
elif [ $MODELSIZE -eq 9 ] then
    echo "Vocab size is 9, setting to 16k"
    MODELSIZE=16k
elif [ $MODELSIZE -eq 10 ] then
    echo "Vocab size is 10, setting to 32k"
    MODELSIZE=32k
elif [ $MODELSIZE -eq 11 ] then
    echo "Vocab size is 11, setting to 64k"
    MODELSIZE=64k
elif [ $MODELSIZE -eq 12 ] then
    echo "Vocab size is 12, setting to 128k"
    MODELSIZE=128k
else
    echo "Unrecognized vocab size"
    echo "Valid sizes are 8, 9, 10, 11, 12"
    exit 1
fi
BPEFILE="bpe-$MODELSIZE.model"
UNIFILE="uni-$MODELSIZE.model"

echo "BPE file: $BPEFILE"
echo "UNI file: $UNIFILE"

