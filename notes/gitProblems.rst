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