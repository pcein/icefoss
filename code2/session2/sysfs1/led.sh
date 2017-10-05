
echo 0 > /sys/class/gpio/export

echo out > /sys/class/gpio/gpio0/direction

# Put on LED
echo 1 > /sys/class/gpio/gpio0/value
