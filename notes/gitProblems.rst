git remote -v
origin  https://github.com/qslia/candle.git (fetch)
origin  https://github.com/qslia/candle.git (push)

git remote add upstream https://github.com/huggingface/candle.git

git remote -v
origin  https://github.com/qslia/candle.git (fetch)
origin  https://github.com/qslia/candle.git (push)
upstream        https://github.com/huggingface/candle.git (fetch)
upstream        https://github.com/huggingface/candle.git (push)

git switch -c my-work
Switched to a new branch 'my-work'

git branch -a
  main
* my-work
  remotes/origin/HEAD -> origin/main
  remotes/origin/main

git push -u origin my-work

Enumerating objects: 18, done.
Counting objects: 100% (18/18), done.
Delta compression using up to 24 threads
Compressing objects: 100% (11/11), done.
Writing objects: 100% (13/13), 6.86 KiB | 6.86 MiB/s, done.
Total 13 (delta 5), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (5/5), completed with 5 local objects.
remote:
remote: Create a pull request for 'my-work' on GitHub by visiting:
remote:      https://github.com/qslia/candle/pull/new/my-work
remote:
To https://github.com/qslia/candle.git
 * [new branch]        my-work -> my-work
branch 'my-work' set up to track 'origin/my-work'.

git branch -a
  main
* my-work
  remotes/origin/HEAD -> origin/main
  remotes/origin/main
  remotes/origin/my-work

git fetch upstream
remote: Enumerating objects: 10253, done.
remote: Counting objects: 100% (5171/5171), done.
remote: Compressing objects: 100% (1988/1988), done.
remote: Total 4129 (delta 2900), reused 3188 (delta 2091), pack-reused 0 (from 0)
Receiving objects: 100% (4129/4129), 3.92 MiB | 16.80 MiB/s, done.
Resolving deltas: 100% (2900/2900), completed with 477 local objects.
From https://github.com/huggingface/candle
 * [new branch]        arrayvec-shape-stride-impl -> upstream/arrayvec-shape-stride-impl
 * [new branch]        backpropagation        -> upstream/backpropagation

git branch -a
main
* my-work
  remotes/origin/HEAD -> origin/main
  remotes/origin/main
  remotes/origin/my-work
  remotes/upstream/arrayvec-shape-stride-impl
  remotes/upstream/backpropagation

git switch main
Switched to branch 'main'
Your branch is ahead of 'origin/main' by 1 commit.

git reset --hard upstream/main
HEAD is now at 701205a3 Update dependencies (#3135)
git remote -v
origin  https://github.com/qslia/candle.git (fetch)
origin  https://github.com/qslia/candle.git (push)

git remote add upstream https://github.com/huggingface/candle.git

git remote -v
origin  https://github.com/qslia/candle.git (fetch)
origin  https://github.com/qslia/candle.git (push)
upstream        https://github.com/huggingface/candle.git (fetch)
upstream        https://github.com/huggingface/candle.git (push)

git switch -c my-work
Switched to a new branch 'my-work'

git branch -a
  main
* my-work
  remotes/origin/HEAD -> origin/main
  remotes/origin/main

git push -u origin my-work

Enumerating objects: 18, done.
Counting objects: 100% (18/18), done.
Delta compression using up to 24 threads
Compressing objects: 100% (11/11), done.
Writing objects: 100% (13/13), 6.86 KiB | 6.86 MiB/s, done.
Total 13 (delta 5), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (5/5), completed with 5 local objects.
remote:
remote: Create a pull request for 'my-work' on GitHub by visiting:
remote:      https://github.com/qslia/candle/pull/new/my-work
remote:
To https://github.com/qslia/candle.git
 * [new branch]        my-work -> my-work
branch 'my-work' set up to track 'origin/my-work'.