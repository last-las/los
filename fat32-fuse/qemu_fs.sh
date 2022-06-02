dd if=/dev/zero of=../user/target/riscv64gc-unknown-none-elf/release/fs.img bs=512 count=16384
sudo mkfs.vfat -F 32 ../user/target/riscv64gc-unknown-none-elf/release/fs.img
sudo chmod 777 ../user/target/riscv64gc-unknown-none-elf/release/fs.img
#sudo mount fat32.img sd_mnt
#sudo chmod 777 sd_mntS