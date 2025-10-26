git add .
git commit -m "publish"
git push -u origin main
ssh quinn@serverus "cd /home/quinn/projects/fang_rs; git fetch; git pull; docker build . -t fang; docker save fang > builds/fang.tar; scp builds/fang.tar root@qcloud-1:/root/docker/nginx/sites/fang_rs/builds"
