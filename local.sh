
if [ "$1" == "set" ] 
then 
ip address add 192.168.10.3/24 dev eno1 
ip route add 192.168.10.0/24 dev eno1 
elif [ "$1" == "del" ] 
then 
ip address del 192.168.10.3/24 dev eno1 
ip route del 192.168.10.0/24 dev eno1 
else
echo "No action specified"
fi

