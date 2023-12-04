#!/bin/bash

# Define the name of your program
PROGRAM_NAME="copilot2chat"

start_program() {
    echo "Starting $PROGRAM_NAME..."
    read -p "Enter GHU_TOKEN: " GHU_TOKEN
    export GHU_TOKEN=$GHU_TOKEN
    nohup ./$PROGRAM_NAME &
    echo "Started $PROGRAM_NAME..."
    echo "PID: $!"
}

stop_program() {
    echo "Stopping $PROGRAM_NAME..."
    pkill -f $PROGRAM_NAME
}

restart_program() {
    echo "Restarting $PROGRAM_NAME..."
    stop_program
    start_program
}

case $1 in
    start)
        start_program
        ;;
    stop)
        stop_program
        ;;
    restart)
        restart_program
        ;;
    *)
        echo "Usage: $0 {start|stop|restart}"
esac
