#!/usr/bin/env sh

set -e

[ -z "${INPUT_TEMPLATE}" ] && (echo "❌ Missing template."; exit 1)
[ -z "${INPUT_FEEDS}" ] && (echo "❌ Missing feeds."; exit 1)

echo -n "Pulling reader feed... "
git config --global user.name "${GITHUB_ACTOR}"
git config --global user.email "${GITHUB_ACTOR}@users.noreply.github.com"
git clone --quiet --depth=1 --single-branch --branch "${INPUT_BRANCH}" "https://x-access-token:${INPUT_GITHUBTOKEN}@github.com/${GITHUB_REPOSITORY}.git" /tmp/reader
cd /tmp/reader
echo "✅"

echo -n "Updating reader feed... "
/github-rss-reader "${INPUT_TEMPLATE}" "${INPUT_FEEDS}" 1> index.html
echo "✅"

echo -n "Publishing reader feed... "
git add -A && git commit --quiet --allow-empty -am "Update reader feed at $(date -u)"
git push --quiet --force
echo "✅"

echo "Successfully updated reader feed 🎉"
