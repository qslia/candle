
git remote add upstream https://github.com/huggingface/candle.git
git remote -v

git switch -c my-work
git push -u origin my-work

git fetch upstream
git switch main
git reset --hard upstream/main



git stash push -m "temp save before switching branches"
git switch my-work
git stash pop




