# 自动将文档push到github的vitefest-docs仓库
git clone git@github.com:Vincent-the-gamer/vitefest-docs.git
pnpm run build
cd contents/.vitepress/dist
cp -r . ../../../vitefest-docs
cd ../../../vitefest-docs

# 现在的时间
time_now=`date "+%Y年%m月%d日---%H时%M分%S秒"`

git add .
git commit -m "部署: "${time_now}
git push origin main

cd ../
rm -rf vitefest-docs
rm -rf contents/.vitepress/dist