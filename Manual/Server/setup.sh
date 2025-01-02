# have to restart the kernel after this
# sudo sysctl -w net.ipv4.ip_forward = 1  

# this would be reset in the next session unless the file /etc/sysctl.conf is modified

echo "net.ipv4.ip_forward = 1" >>/etc/sysctl.conf
echo "net.ipv4.conf.all.proxy_arp = 1" >>/etc/sysctl.conf

# use sudo if you face permission issues 