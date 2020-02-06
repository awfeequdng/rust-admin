#!/bin/bash

#host='locahost'
name='rust_admin'
user='rust_admin'
password='rust-x-lsl'

sql_file="`date '+%Y%M%d'`.SQL"
if [ -f $sql_file ]; then
    rm -rf $sql_file
fi

mysqldump -u$user -p"$password" $name > $sql_file
