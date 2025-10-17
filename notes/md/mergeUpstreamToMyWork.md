Great question. You’ve got two solid choices to bring **upstream/main** into **my-work**. Pick one style and stick with it.

---

## Option A — Rebase (clean, linear history) ✅ *recommended for feature branches*

```bash
# 1) Update your local view of upstream
git fetch upstream

# 2) Make sure your branch is current and clean
git switch my-work

# 3) Replay your commits on top of upstream/main
git rebase upstream/main
```

* If you hit conflicts:

  ```bash
  # edit files to resolve, then:
  git add <file>
  git rebase --continue
  # to back out:
  git rebase --abort
  ```
* After a successful rebase, push the updated branch history:

  ```bash
  git push --force-with-lease origin my-work
  ```

  (`--force-with-lease` is safe; it refuses to overwrite others’ work.)

**Variant (nice flow):** keep `main` mirrored to upstream, then rebase on `main`:

```bash
git switch main
git pull --ff-only         # pulls from upstream/main if you set that earlier
git switch my-work
git rebase main
git push --force-with-lease origin my-work
```

---

## Option B — Merge (preserves original commit shapes)

```bash
git fetch upstream
git switch my-work
git merge upstream/main
# resolve conflicts -> git add <file>; then:
git commit                  # completes the merge
git push origin my-work
```

No history rewrite, so no force-push needed. You’ll get a merge commit.

---

## Tips & conflict helpers

* Stash local dirt before syncing:

  ```bash
  git stash push -m "wip"
  # ... do the rebase/merge ...
  git stash pop
  ```
* During conflict resolution:

  ```bash
  # keep your version:
  git checkout --ours -- path/to/file
  # keep upstream version:
  git checkout --theirs -- path/to/file
  ```
* Make Git remember repeated resolutions:

  ```bash
  git config --global rerere.enabled true
  ```

---

## Quick chooser

| Goal                                    | Use        |
| --------------------------------------- | ---------- |
| Keep history linear, easy to review     | **Rebase** |
| Avoid rewriting history (shared branch) | **Merge**  |

If this is just your feature branch, I’d rebase onto `upstream/main` (Option A) and force-with-lease push.
