tailwindcss:
    bunx @tailwindcss/cli -i ./styles/tailwind.css -o ./public/styles.css --watch

server:
    cargo watch -i styles -i public -i uploads -c -x 'run'

