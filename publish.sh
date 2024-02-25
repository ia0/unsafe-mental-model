#!/bin/sh
set -e

d() { echo "[1;32mDone:[m $*"; exit 0; }
e() { echo "[1;31mError:[m $*"; exit 1; }

[ -z "$(git status -s)" ] || e 'not clean'
if [ -z "$CI" ]; then
  [ "$(git symbolic-ref -q HEAD)" = refs/heads/main ] || e 'not main'
else
  set -x
  git config --global user.name ia0
  git config --global user.email git@ia0.eu
  [ "$GITHUB_REF" = refs/heads/main ] || e 'not main'
fi
COMMIT="$(git rev-parse -q --verify HEAD)"
[ -n "$COMMIT" ] || e 'failed to get commit hash'

git diff --quiet "$(git log --pretty=format:%f origin/gh-pages)".. -- book.toml src \
  && d "origin/gh-pages is already up-to-date"

which mdbook >/dev/null 2>&1 || cargo install mdbook
mdbook build

git show-ref -q --verify refs/heads/gh-pages && git branch -qD gh-pages
git checkout -q --orphan gh-pages
git rm -qrf .
git clean -qfxde/book
find book -mindepth 1 -maxdepth 1 -exec mv {} . \;
rmdir book
git add .
git commit -qm"$COMMIT"
git checkout -q main
[ -z "$CI" ] || git push -f origin gh-pages
d "gh-pages has been updated"
