#! /bin/bash
alias k-tdt="sudo lsof -i -P -n | grep LISTEN | grep todo_tags | cut -d ' ' -f 2 | xargs kill"
alias s-tdt="sudo lsof -i -P -n | grep LISTEN | grep todo_tags"
