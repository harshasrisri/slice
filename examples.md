# Examples

The following options apply to all examples:
* Delimiter = '|' 
* Separator = ',' 

## First
* Skip lines without delimiter
* Use fields specified as-is
<!-- `$ echo \$ slice inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s "," && cargo run --quiet --release -- inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s ","` -->
```
$ slice inputs/small_pipe.txt -d | -f -3,12-15,6-9,22,30- -s ,
Suspendisse,blandit,eros,luctus,Donec,at,egestas,egestas,nunc,sit,amet,Duis,id,turpis,Maecenas,rhoncus,bibendum,mi,eget,tristique,erat,vehicula,et,Duis,ultricies,laoreet,massa,Duis,ullamcorper,justo,eget,diam,mattis,ac,tincidunt,odio,suscipit,Aenean,feugiat,purus,vitae,quam,congue,tristique,pretium,mauris,placerat,Nam,id,fermentum,justo,Praesent,lobortis,sollicitudin,neque,nec,pretium,arcu,porttitor,sit,amet,Sed,et,euismod,libero,imperdiet,sollicitudin,ligula,Curabitur,accumsan,eu,ante,sed,faucibus,Donec,placerat,eros,sit,amet,libero,interdum,in,varius,leo,porttitor,Mauris,non,congue,metus,nec,posuere,nulla,Vivamus,ex,leo,sollicitudin,iaculis,tincidunt,ac,aliquam,sed,urna
Nulla,maximus,leo,accumsan,ut,lacinia,diam,nunc,lacus,elementum,eget,a,justo,at,ligula,dapibus,dictum,in,eu,justo,Maecenas,finibus,fringilla,laoreet,Mauris,nec,ante,semper,ultrices,lorem,ut,tincidunt,lacus,Morbi,dapibus,lobortis,efficitur
Mauris,vel,purus,finibus,egestas,non,eu,hendrerit,augue,ut,cursus,imperdiet,ac,maximus,porttitor,In,et,velit,eleifend,dictum,turpis,et,interdum,metus,Phasellus,ligula,risus,fermentum,at,dui,pellentesque,mollis,ultricies,dui,Duis,et,faucibus,metus,Maecenas,cursus,tincidunt,finibus,Nunc,eget,aliquam,justo,Proin,eros,tortor,sollicitudin,semper,condimentum,sed,hendrerit,at,urna,Maecenas,lobortis,nisi,vel,scelerisque,tristique,eros,libero,condimentum,urna,vel,porta,mi,purus,id,tellus,In,vel,orci,nunc,Curabitur,tempus,vehicula,eros,sed,varius,Curabitur,luctus,ipsum,eget,hendrerit,volutpat,leo,sapien,convallis,mi,tempor,fringilla,metus,odio,cursus,metus
Nulla,tempor,auctor,pellentesque,augue,blandit,a,eros,ac,egestas,scelerisque,ex,enim,Morbi,volutpat,et,orci,nec,feugiat,Nunc,sodales,rutrum,leo,in,dignissim,Curabitur,egestas,id,nibh,eu,porta,Vestibulum,sodales,mattis,dolor,quis,faucibus,ante,mattis,vel,Donec,hendrerit,rutrum,ante,sit,amet,lacinia,Maecenas,in,ligula,tortor,In,fringilla,dolor,in,sagittis,lobortis,lorem,elit,dictum,eros,ultrices,placerat,felis,libero,ut,sapien,Aliquam,in,orci,ut,nulla,ullamcorper,posuere,Praesent,laoreet,consequat,tellus,a,ornare,Etiam,tempor,nunc,convallis,sollicitudin,bibendum,Morbi,a,ligula,erat,Nulla,hendrerit,ipsum,viverra,pretium,libero,finibus,semper,sapien,Integer,sit,amet,est,ipsum
Ut,pellentesque,mi,accumsan,finibus,Sed,lobortis,volutpat,viverra,Suspendisse,suscipit,quis,mattis,sed,Phasellus,tempor,leo,non,luctus,porttitor,dui,sapien,eleifend,sem,a,blandit,sapien,nibh,in,purus,Donec,pharetra,neque,vitae,semper,pellentesque,Morbi,ut,tristique,metus
Aliquam,eget,justo,fringilla,tincidunt,dolor,eu,commodo,eget,Mauris,tristique,tortor,vitae,rutrum,lorem,molestie,Proin,lectus,sem,lobortis,eu,mattis,ut,vulputate,sed,ex,In,magna,ligula,ullamcorper,a,interdum,quis,egestas,sed,ligula,Suspendisse,elementum,dui,vel,ultricies,tincidunt,odio,mauris,viverra,dolor,nec,laoreet,arcu,est,sollicitudin,magna
Nam,aliquet,purus,tristique,velit,consequat,vitae,leo,nec,urna,sodales,Curabitur,varius,eros,Curabitur,dapibus,sem,ut,nisl,euismod,faucibus,Sed,interdum,ex,vitae,purus,mattis,iaculis,Sed,feugiat,a,orci,eu,imperdiet,Duis,et,tortor,id,orci,fermentum,sollicitudin,Donec,ultricies,orci,vitae,quam,congue,ut,ultrices,ex,euismod,Nullam,tempor,viverra,mi,sed,auctor,est,dapibus,sed
Phasellus,luctus,efficitur,fringilla,mi,a,volutpat,libero,porttitor,sem,non,luctus,pellentesque,enim,sem,tincidunt,ex,vitae,fermentum,urna,nulla,id,ipsum,Duis,dignissim,lacus,id,varius,malesuada,Pellentesque,elit,ante,placerat,non,bibendum,vel,ultricies,eu,quam,Praesent,id,lacus,efficitur,pretium,massa,quis,efficitur,sem,Curabitur,porttitor,turpis,vitae,eros,euismod,convallis
Integer,pretium,velit,congue,purus,dapibus,faucibus,imperdiet,libero,Pellentesque,placerat,posuere,lorem,Maecenas,at,libero,pretium,fermentum,purus,eget,suscipit,nisl,Suspendisse,finibus,in,velit,non,imperdiet,Suspendisse,potenti,Ut,mollis,eu,nisl,vitae,ornare,Praesent,sit,amet,orci,at,magna,interdum,hendrerit,Pellentesque,vulputate,eu,mauris,a,condimentum,Nullam,lectus,erat,tempor,ut,commodo,tempus,venenatis,sed,tellus,Mauris,aliquet,metus,nec,orci,porttitor,eleifend,Nullam,non,lectus,nisl
Suspendisse,tellus,nunc,varius,in,fermentum,non,vitae,facilisis,nunc,Donec,in,volutpat,sit,amet,mattis,nisl,dictum,Duis,a,consequat,purus,sit,amet,finibus,tortor,Praesent,viverra,pretium,augue,et,ullamcorper,tellus,cursus,eget,Vestibulum,luctus,erat,nibh,vel,cursus,dolor,lacinia,et,Vestibulum,orci,quam,euismod,ac,diam,id,tristique,condimentum,purus,Aliquam,efficitur,lacus,ac,velit,mattis,sit,amet,lobortis,velit,dapibus
In,mattis,fermentum,cursus,Suspendisse,eget,eros,sapien,dictum,accumsan,non,enim,ipsum,erat,commodo,ut,est,molestie,tristique,faucibus,magna,Vestibulum,lacinia,blandit,dolor,vel,dictum,Nunc,suscipit,libero,nunc,nec,sollicitudin,nulla,convallis,a,Etiam,mollis,elit,in,convallis,laoreet,Aliquam,ante,velit,mollis,nec,aliquam,quis,posuere,a,lectus,Nullam,nunc,ipsum,convallis,laoreet,sem,eget,cursus,ullamcorper,enim,Etiam,quis,odio,eu,leo,imperdiet,suscipit,Nullam,eu,dignissim,mi,non,hendrerit,sem,Pellentesque,habitant,morbi,tristique,senectus,et,netus,et,malesuada,fames,ac,turpis,egestas,Pellentesque,habitant,morbi,tristique,senectus,et,netus,et,malesuada,fames,ac,turpis,egestas,Curabitur,semper,lacus,ultricies,felis,malesuada,tincidunt
Etiam,vestibulum,arcu,condimentum,arcu,pharetra,eu,libero,eu,ipsum,mollis,rutrum,tincidunt,dictum,enim,non,posuere,justo,fringilla,eget,Aliquam,eu,augue,et,massa,aliquam,molestie,Proin,eu,risus,in,nisl,venenatis,eleifend,dignissim,quis,magna,In,condimentum,nisi,in,tincidunt,volutpat,nisl,orci,rhoncus,mauris,non,aliquet,enim,sem,ac,mi,Vestibulum,sit,amet,iaculis,arcu,at,commodo,ligula,In,hac,habitasse,platea,dictumst,Aliquam,erat,volutpat,Cras,hendrerit,iaculis,interdum,Ut,at,sapien,quis,sapien,tristique,interdum,et,placerat,purus
Nullam,euismod,ex,tincidunt,cursus,Nullam,ornare,dolor,malesuada,tincidunt,Nullam,tortor,sem,Cras,commodo,enim,eget,imperdiet,scelerisque,massa,lacus,vestibulum,risus,sed,tempor,purus,lacus,nec,tellus,Pellentesque,fermentum,porttitor,dui,et,efficitur,magna,congue,non,Integer,arcu,ligula,imperdiet,nec,tortor,eu,pretium,rutrum,odio,Integer,nec,porta,lorem,nec,interdum,orci,Curabitur,laoreet,consectetur,ullamcorper,Cras,id,diam,dignissim,tincidunt,felis,non,pellentesque,purus,Aliquam,tincidunt,id,sem,ut,rutrum,Class,aptent,taciti,sociosqu,ad,litora,torquent,per,conubia,nostra,per,inceptos,himenaeos,Nulla,mauris,dolor,posuere,sit,amet,quam,et,fermentum,efficitur,dui,Quisque,sed,risus,non,enim,semper,euismod,id,a,enim,Mauris,pellentesque,libero,vitae,semper,egestas,quam,est,tempor,ipsum,ut,aliquam,nibh,velit,ac,massa
Suspendisse,gravida,sapien,faucibus,sed,pretium,felis,ut,mauris,in,tortor,porta,Ut,consequat,sollicitudin,laoreet,Pellentesque,molestie,nibh,eget,tortor,tempus,pellentesque,eget,sit,amet,lacus,Sed,vel,ex,et,erat,porta,feugiat,Duis,rhoncus,leo,a,lobortis,sollicitudin,Pellentesque,vestibulum,nibh,a,eleifend,lobortis,quam,sapien,finibus,tellus,eget,porttitor,libero,ante,vel,sapien,Etiam,ac,magna,tellus,Ut,et,lectus,efficitur,ultrices,sem,sed,sollicitudin,velit,Donec,varius,ante,ac,ante,iaculis,blandit,Nullam,porttitor,est,augue,ac,tincidunt,odio,venenatis,vitae,Aenean,vulputate,bibendum,ornare,Vivamus,pharetra,libero,vitae,luctus,pharetra,libero,massa,tincidunt,ex,et,molestie,est,purus,eget,massa
Maecenas,sodales,ipsum,posuere,elit,vestibulum,dictum,turpis,vitae,ullamcorper,viverra,neque,sed,volutpat,tortor,lobortis,et,Phasellus,lacinia,libero,sed,leo,tempor,eu,porta,quam,mollis,Donec,nec,lacinia,magna,Nulla,consectetur,fermentum,erat,sit,amet,commodo,mi,bibendum,sit,amet,Vivamus,varius,augue,vitae,fermentum,pulvinar
Phasellus,faucibus,tempus,maximus,diam,pulvinar,sed,eleifend,lectus,Fusce,tincidunt,eu,dolor,sit,amet,consectetur,adipiscing,elit,Nulla,facilisi,Maecenas,aliquam,justo,in,placerat,feugiat,Aliquam,commodo,sed,massa,ac,iaculis,Etiam,a,dignissim,justo,et,bibendum,libero
Nulla,quis,pulvinar,sit,amet,ex,vitae,sollicitudin,at,non,tortor,amet,Duis,at,felis,in,felis,ultrices,dignissim,sed,convallis,magna,Pellentesque,habitant,morbi,tristique,senectus,et,netus,et,malesuada,fames,ac,turpis,egestas,Duis,in,vulputate,diam,Maecenas,eget,risus,faucibus,tempor,arcu,placerat,tincidunt,sem,Mauris,libero,libero,elementum,sed,dignissim,nec,pretium,dignissim,lorem,Etiam,ut,feugiat,erat,Phasellus,ullamcorper,a,erat,at,posuere,Morbi,sagittis,egestas,nisi,eget,dignissim,justo,auctor,eget,Duis,porttitor,nec,magna,vitae,vestibulum,Suspendisse,potenti
Sed,tincidunt,cursus,tincidunt,dolor,sed,mi,mollis,nisl,facilisis,Etiam,Fusce,elit,mollis,magna,nec,efficitur,enim,diam,id,augue,Orci,varius,natoque,penatibus,et,magnis,dis,parturient,montes,nascetur,ridiculus,mus,Proin,at,ligula,ut,mi,vestibulum,varius,Etiam,quam,nulla,ullamcorper,in,tempus,vel,pulvinar,nec,dui
Maecenas,id,semper,semper,ipsum,Nunc,cursus,at,lobortis,justo,dapibus,porttitor,eu,tempor,feugiat,purus,elit,aliquam,velit,at,interdum,nibh,sem,a,justo,Suspendisse,ultricies,interdum,ligula,sed,aliquet,metus,lacinia,ac,Sed,interdum,lacus,interdum,libero,finibus,pretium,aliquam,lectus,mattis,Mauris,tincidunt,leo,euismod,nunc,sagittis,aliquam,nec,at,dui,Quisque,placerat,hendrerit,ex,eu,hendrerit,magna,fringilla,eu,Nullam,ut,nunc,ut,tortor,sollicitudin,hendrerit,Morbi,sed,sagittis,lorem,Sed,consequat,augue,in,ex,consequat,in,vestibulum,dui,facilisis
Maecenas,dapibus,dapibus,aliquam,Class,aptent,taciti,litora,torquent,per,conubia,malesuada,Curabitur,in,ex,eget,mi,pretium,pretium,Ut,vulputate,quis,sapien,at,feugiat,Praesent,eget,mi,ex,Ut,non,consectetur,sem,at,tempus,arcu
```

## Second
* Skip lines without delimiter
* Use inverse / complement of fields specified
<!-- `$ echo \$ slice inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s "," -c && cargo run --quiet --release -- inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s "," -c` -->
```
$ slice inputs/small_pipe.txt -d | -f -3,12-15,6-9,22,30- -s , -c
lobortis,varius,nunc,Nulla,nisl,suscipit,sed,ultrices,nunc,ornare,sed,nisi,a,orci,luctus,venenatis,eget
cursus,erat,elementum,Donec,justo,eget,congue,lobortis,felis,Quisque,lorem,nec,massa,faucibus,vestibulum,Maecenas,vel
vel,dolor,tellus,Mauris,rutrum,Nunc,tincidunt,eros,non,ex,quis,mollis,lacus,mattis,Vestibulum,commodo,metus
nulla,et,Duis,tempus,enim,magna,sollicitudin,quam,sed,eleifend,est,ac,mauris,Proin,sit,amet,mi
at,lectus,erat,sed,feugiat,nibh,nec,vehicula,libero,condimentum,Donec,luctus,turpis,felis,ut,sagittis,odio
purus,Ut,sodales,felis,massa,ut,massa,venenatis,tincidunt,porttitor,porttitor,Sed,condimentum,justo,eu,est,ullamcorper
elit,eleifend,Maecenas,sodales,elementum,Suspendisse,potenti,Aliquam,erat,volutpat,orci,nunc,pellentesque,non,quam,lobortis,feugiat
sagittis,Cras,commodo,nibh,congue,nunc,sem,et,nulla,Proin,hendrerit,maximus,Phasellus,consequat,nisl,at,posuere
lectus,quis,Pellentesque,eu,neque,eu,congue,rutrum,massa,augue,dolor,sit,amet,rutrum,turpis,risus,vel
consectetur,eget,ex,Morbi,sed,enim,eget,ligula,tempus,malesuada,quis,dolor,Morbi,vestibulum,nunc,imperdiet,elit
leo,eleifend,sit,amet,eget,elit,Sed,ut,justo,vel,tincidunt,venenatis,Fusce,a,lobortis,ex,Sed
eros,in,Aliquam,vel,auctor,in,nec,ex,Maecenas,consectetur,est,ut,hendrerit,urna,egestas,a,Donec
at,ligula,ipsum,in,pellentesque,interdum,lacus,vitae,viverra,Mauris,nunc,convallis,pellentesque,pellentesque,in,finibus,ac
at,enim,sollicitudin,Sed,tristique,facilisis,sed,ac,tellus,Pellentesque,finibus,nulla,eget,tincidunt,elit,mattis,nec
quam,a,Donec,iaculis,tortor,magna,viverra,nibh,a,lobortis,diam,a,elit,Donec,congue,nisi,lectus
justo,vel,Donec,nec,porttitor,placerat,Donec,ante,est,ultrices,ligula,ut,convallis,blandit,ex,Lorem,ipsum
diam,In,enim,pulvinar,Sed,eget,nunc,sem,Nullam,sit,libero,euismod,tristique,leo,in,rhoncus,sem
rhoncus,Sed,volutpat,at,nec,leo,nec,tellus,euismod,consectetur,lobortis,orci,sit,amet,accumsan,posuere,orci
dui,id,urna,sem,ut,Integer,augue,nibh,ultrices,id,ut,accumsan,at,odio,Vivamus,elementum,mauris
diam,ut,sociosqu,ad,nostra,per,inceptos,himenaeos,Interdum,et,fames,ac,ante,ipsum,primis,in,faucibus
```

## Third
* Include lines without delimiter
* Use inverse / complement of fields specified
<!-- `$ echo \$ slice inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s "," -c -n && cargo run --quiet --release -- inputs/small_pipe.txt -d "|" -f -3,12-15,6-9,22,30- -s "," -c -n` -->
```
$ slice inputs/small_pipe.txt -d | -f -3,12-15,6-9,22,30- -s , -c -n
lobortis,varius,nunc,Nulla,nisl,suscipit,sed,ultrices,nunc,ornare,sed,nisi,a,orci,luctus,venenatis,eget

cursus,erat,elementum,Donec,justo,eget,congue,lobortis,felis,Quisque,lorem,nec,massa,faucibus,vestibulum,Maecenas,vel

vel,dolor,tellus,Mauris,rutrum,Nunc,tincidunt,eros,non,ex,quis,mollis,lacus,mattis,Vestibulum,commodo,metus

nulla,et,Duis,tempus,enim,magna,sollicitudin,quam,sed,eleifend,est,ac,mauris,Proin,sit,amet,mi

at,lectus,erat,sed,feugiat,nibh,nec,vehicula,libero,condimentum,Donec,luctus,turpis,felis,ut,sagittis,odio

purus,Ut,sodales,felis,massa,ut,massa,venenatis,tincidunt,porttitor,porttitor,Sed,condimentum,justo,eu,est,ullamcorper

elit,eleifend,Maecenas,sodales,elementum,Suspendisse,potenti,Aliquam,erat,volutpat,orci,nunc,pellentesque,non,quam,lobortis,feugiat

sagittis,Cras,commodo,nibh,congue,nunc,sem,et,nulla,Proin,hendrerit,maximus,Phasellus,consequat,nisl,at,posuere

lectus,quis,Pellentesque,eu,neque,eu,congue,rutrum,massa,augue,dolor,sit,amet,rutrum,turpis,risus,vel

consectetur,eget,ex,Morbi,sed,enim,eget,ligula,tempus,malesuada,quis,dolor,Morbi,vestibulum,nunc,imperdiet,elit

leo,eleifend,sit,amet,eget,elit,Sed,ut,justo,vel,tincidunt,venenatis,Fusce,a,lobortis,ex,Sed

eros,in,Aliquam,vel,auctor,in,nec,ex,Maecenas,consectetur,est,ut,hendrerit,urna,egestas,a,Donec

at,ligula,ipsum,in,pellentesque,interdum,lacus,vitae,viverra,Mauris,nunc,convallis,pellentesque,pellentesque,in,finibus,ac

at,enim,sollicitudin,Sed,tristique,facilisis,sed,ac,tellus,Pellentesque,finibus,nulla,eget,tincidunt,elit,mattis,nec

quam,a,Donec,iaculis,tortor,magna,viverra,nibh,a,lobortis,diam,a,elit,Donec,congue,nisi,lectus

justo,vel,Donec,nec,porttitor,placerat,Donec,ante,est,ultrices,ligula,ut,convallis,blandit,ex,Lorem,ipsum

diam,In,enim,pulvinar,Sed,eget,nunc,sem,Nullam,sit,libero,euismod,tristique,leo,in,rhoncus,sem

rhoncus,Sed,volutpat,at,nec,leo,nec,tellus,euismod,consectetur,lobortis,orci,sit,amet,accumsan,posuere,orci

dui,id,urna,sem,ut,Integer,augue,nibh,ultrices,id,ut,accumsan,at,odio,Vivamus,elementum,mauris

diam,ut,sociosqu,ad,nostra,per,inceptos,himenaeos,Interdum,et,fames,ac,ante,ipsum,primis,in,faucibus

```
