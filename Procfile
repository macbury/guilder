watch-file-change:    cargo watch --why --watch src --watch migration --watch entity --watch data_sources -x check -s 'touch restart.txt'
app:                  cargo watch -N --no-gitignore -w restart.txt -x run
frontend:             cd frontend && yarn dev
