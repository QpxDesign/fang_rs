docker build . -t fang --platform linux/amd64
docker save fang > builds/fang.tar
git add .
git commit -m "publish"
git push -u serverus main
