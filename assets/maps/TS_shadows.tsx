<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.12.1" name="Shadows" tilewidth="64" tileheight="64" tilecount="15" columns="5">
 <image source="../tilesets/tilesets/stone/Shadows.png" trans="ff00ff" width="320" height="192"/>
 <tile id="0">
  <objectgroup draworder="index" id="3">
   <object id="2" x="63.875" y="32.25">
    <polygon points="0,0 -9.625,0 -21.875,6 -25.75,9.75 -29.75,13.75 -31.875,17.75 -31.75,31.5 0.25,31.625"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="1">
  <objectgroup draworder="index" id="2">
   <object id="1" x="-0.181818" y="31.6364">
    <polygon points="0,0 64,0.181818 64.1818,32 0.181818,32.1818"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="2">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0.325624" y="32.3636">
    <polygon points="0,0 9.47956,0 21.5445,6 25.3609,9.75 29.3005,13.75 31.3934,17.75 31.2702,31.5 -0.246222,31.625"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="3">
  <objectgroup draworder="index" id="4">
   <object id="3" x="32.5455" y="63.4545">
    <polygon points="0,0 0.727273,-13.4545 3.63636,-18.9091 10.9091,-25.6364 19.8182,-29.8182 22.3636,-31.2727 31.6364,-31.2727 31.4545,-63.6364 -32.3636,-63.6364 -32.7273,-0.545455 -32.5455,0.545455"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="4">
  <objectgroup draworder="index" id="2">
   <object id="1" x="31.8182" y="63.4545">
    <polygon points="0,0 -0.727273,-13.4545 -3.63636,-18.9091 -10.9091,-25.6364 -19.8182,-29.8182 -22.3636,-31.2727 -31.6364,-31.2727 -31.4545,-63.6364 32.3636,-63.6364 32.7273,-0.545455 32.5455,0.545455"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="5">
  <objectgroup draworder="index" id="2">
   <object id="1" x="31.4545" y="0">
    <polygon points="0,0 31.4545,0 31.4545,63.4545 -0.363636,63.6364"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="6">
  <objectgroup draworder="index" id="2">
   <object id="2" x="0" y="0" width="64" height="64"/>
   <object id="3" x="2" y="0.909091" width="62" height="62.5455"/>
  </objectgroup>
 </tile>
 <tile id="7">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0.545455" y="0.181818">
    <polygon points="0,0 31.4545,0 31.4545,63.4545 -0.363636,63.6364"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="8">
  <objectgroup draworder="index" id="2">
   <object id="1" x="32.3636" y="0.3636">
    <polygon points="0,0 0.727273,13.4545 3.63636,18.9091 10.9091,25.6364 19.8182,29.8182 22.3636,31.2727 31.6364,31.2727 31.4545,63.6364 -32.3636,63.6364 -32.7273,0.545455 -32.5455,-0.545455"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="9">
  <objectgroup draworder="index" id="2">
   <object id="1" x="31.0909" y="0.909055">
    <polygon points="0,0 -0.727273,13.4545 -3.63636,18.9091 -10.9091,25.6364 -19.8182,29.8182 -22.3636,31.2727 -31.6364,31.2727 -31.4545,63.6364 32.3636,63.6364 32.7273,0.545455 32.5455,-0.545455"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="10">
  <objectgroup draworder="index" id="2">
   <object id="1" x="62.4199" y="32.1705">
    <polygon points="0,0 -9.47956,0 -21.5445,-6 -25.3609,-9.75 -29.3005,-13.75 -31.3934,-17.75 -31.2702,-31.5 0.246222,-31.625"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="11">
  <objectgroup draworder="index" id="4">
   <object id="3" x="0" y="0">
    <polygon points="0,0 0,30.9091 0,31.4545 64.1818,31.4545 63.8182,-0.181818"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="12">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0.363636" y="31.9886">
    <polygon points="0,0 9.47956,0 21.5445,-6 25.3609,-9.75 29.3005,-13.75 31.3934,-17.75 31.2702,-31.5 -0.246222,-31.625"/>
   </object>
  </objectgroup>
 </tile>
 <wangsets>
  <wangset name="shadows_terrain" type="corner" tile="-1">
   <wangcolor name="" color="#ff0000" tile="-1" probability="1"/>
   <wangtile tileid="0" wangid="0,0,0,1,0,0,0,0"/>
   <wangtile tileid="1" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="2" wangid="0,0,0,0,0,1,0,0"/>
   <wangtile tileid="3" wangid="0,1,0,0,0,1,0,1"/>
   <wangtile tileid="4" wangid="0,1,0,1,0,0,0,1"/>
   <wangtile tileid="5" wangid="0,1,0,1,0,0,0,0"/>
   <wangtile tileid="6" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="7" wangid="0,0,0,0,0,1,0,1"/>
   <wangtile tileid="8" wangid="0,0,0,1,0,1,0,1"/>
   <wangtile tileid="9" wangid="0,1,0,1,0,1,0,0"/>
   <wangtile tileid="10" wangid="0,1,0,0,0,0,0,0"/>
   <wangtile tileid="11" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="12" wangid="0,0,0,0,0,0,0,1"/>
  </wangset>
 </wangsets>
</tileset>
