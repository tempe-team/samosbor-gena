# convert UTKFace/81_0_0_20170111211419693.jpg.chip.jpg -resize 24x24 -unsharp 1 -unsharp 1 -remap Warm32/sad32.png -filter point -resize 72x72 out.png


# mogrify -resize 24x24 -unsharp 1 -unsharp 1 -remap ../Warm32/sad32.png -filter point -resize 96x96 -format png ../UTKFace-Sad *.jpg
