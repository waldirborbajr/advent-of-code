# find . -iname target -type d -exec rm -r '{}' \;
find . -name 'target' -type d | xargs rm -r
