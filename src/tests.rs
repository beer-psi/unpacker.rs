use crate::unpack;

#[test]
fn it_works() {
    assert_eq!(
        unpack(r#"eval(function(p,a,c,k,e,r){e=String;if(!''.replace(/^/,String)){while(c--)r[c]=k[c]||c;k=[function(e){return r[e]}];e=function(){return'\\w+'};c=1};while(c--)if(k[c])p=p.replace(new RegExp('\\b'+e(c)+'\\b','g'),k[c]);return p}('(0(){4 1="5 6 7 8";0 2(3){9(3)}2(1)})();',10,10,'function|b|something|a|var|some|sample|packed|code|alert'.split('|'),0,{}))"#),
        r#"(function(){var b="some sample packed code";function something(a){alert(a)}something(b)})();"#,
    );

    assert_eq!(
        unpack(r#"eval(function(p,a,c,k,e,d){e=function(c){return(c<a?"":e(parseInt(c/a)))+((c=c%a)>35?String.fromCharCode(c+29):c.toString(36))};if(!''.replace(/^/,String)){while(c--)d[e(c)]=k[c]||e(c);k=[function(e){return d[e]}];e=function(){return'\\w+'};c=1;};while(c--)if(k[c])p=p.replace(new RegExp('\\b'+e(c)+'\\b','g'),k[c]);return p;}('d E=[\'//c.4.b/3/1/2/9.0/a/w.8?6=v&7=5\',\'//c.4.b/3/1/2/9.0/a/x.8?6=z&7=5\',\'//c.4.b/3/1/2/9.0/a/y.8?6=u&7=5\',\'//c.4.b/3/1/2/9.0/a/q.8?6=p&7=5\',\'//c.4.b/3/1/2/9.0/a/r.8?6=t&7=5\',\'//c.4.b/3/1/2/9.0/a/s.8?6=A&7=5\',\'//c.4.b/3/1/2/9.0/a/I.8?6=H&7=5\',\'//c.4.b/3/1/2/9.0/a/J.8?6=L&7=5\',\'//c.4.b/3/1/2/9.0/a/K.8?6=G&7=5\',\'//c.4.b/3/1/2/9.0/a/C.8?6=B&7=5\',\'//c.4.b/3/1/2/9.0/a/D.8?6=F&7=5\',\'//c.4.b/3/1/2/9.0/a/g.8?6=j&7=5\',\'//c.4.b/3/1/2/9.0/a/h.8?6=i&7=5\',\'//c.4.b/3/1/2/9.0/a/o.8?6=l&7=5\',\'//c.4.b/3/1/2/9.0/a/k.8?6=m&7=5\',\'//c.4.b/3/1/2/9.0/a/n.8?6=f&7=5\',\'//c.4.b/3/1/2/9.0/a/e.8?6=M&7=5\',\'//c.4.b/3/1/2/9.0/a/16.8?6=19&7=5\',\'//c.4.b/3/1/2/9.0/a/18.8?6=1b&7=5\',\'//c.4.b/3/1/2/9.0/a/1a.8?6=15&7=5\',\'//c.4.b/3/1/2/9.0/a/14.8?6=17&7=5\',\'//c.4.b/3/1/2/9.0/a/1h.8?6=1g&7=5\'];d 1j=[1i,1d,1c,1f,1e,S,R,U,T,O,N,Q,P,V,11,10,13,12,X,W,Z,Y];',62,82,'|manga|21619|store|mangafox|1675958400|token|ttl|jpg|126|compressed|me|zjcdn|var|g016|db0af9327aa5f75197427efa4f079be70201cd13|g011|g012|33ddfe0ff12924268a7d20cb63addbcd7d6cdee6|a73649719606232430e76395b8b1e741730d4970|g014|5ac050c927682c56e1a83f226a547f00d4e4faee|e7501a5c573544840d40080d4e60598db9d4addf|g015|g013|b237edd2b93bbf51d4d08b394701baff75f7c31d|g003|g004|g005|bbd3637d291c7b8f2989e17f8f4a7d07eae9bb3a|1a84db22fa4b3241b5f434450d093086e2a35532|c97bea81c2125fcd89951ef5446e6fec7349d09e|g000|g001|g002|b8737f19e4838d92e581e66ac62bf8d46974b0a4|04faaf873ea2f2df6a9f14a74b2d310d53bd4875|35f937d01c1eb0865705e8867c34a609c4679051|g009|g010|newImgs|cbd4776bcb2507efedc5d2be414f9309d664d4a0|5f7e162c8aa9d89e8b6d9c5a093da4bc2c578b24|21ee12733867d2199e62b405bb553a9bd2855f3c|g006|g007|g008|632a12bd8510ad1f6f1361337b14427bd010bc8a|fddda5e1fdaf1c456b386d2cde2e0ae95a3f58c0|28427148|28427147|28427150|28427149|28427144|28427143|28427146|28427145|28427151|28427157|28427156|28427159|28427158|28427153|28427152|28427155|28427154|g020|3b6fdbcbc5a635265d4c6e7ca6ee765928b679da|g017|d5ca636870fd8cec3080a78552521409f30a5c42|g018|d3d1d2571174e1d0e731e3dcfe1533786b66a7ed|g019|3dc78deac0ca8c49de91153de8081c64a547d2b8|28427140|28427139|28427142|28427141|4526d1cb5eebfb3fef3dc90f66a6369cc937a3aa|g021|28427138|newImginfos'.split('|'),0,{}))"#),
        r#"var newImgs=["//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g000.jpg?token=c97bea81c2125fcd89951ef5446e6fec7349d09e&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g001.jpg?token=b8737f19e4838d92e581e66ac62bf8d46974b0a4&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g002.jpg?token=1a84db22fa4b3241b5f434450d093086e2a35532&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g003.jpg?token=b237edd2b93bbf51d4d08b394701baff75f7c31d&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g004.jpg?token=bbd3637d291c7b8f2989e17f8f4a7d07eae9bb3a&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g005.jpg?token=04faaf873ea2f2df6a9f14a74b2d310d53bd4875&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g006.jpg?token=21ee12733867d2199e62b405bb553a9bd2855f3c&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g007.jpg?token=632a12bd8510ad1f6f1361337b14427bd010bc8a&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g008.jpg?token=5f7e162c8aa9d89e8b6d9c5a093da4bc2c578b24&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g009.jpg?token=35f937d01c1eb0865705e8867c34a609c4679051&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g010.jpg?token=cbd4776bcb2507efedc5d2be414f9309d664d4a0&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g011.jpg?token=a73649719606232430e76395b8b1e741730d4970&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g012.jpg?token=33ddfe0ff12924268a7d20cb63addbcd7d6cdee6&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g013.jpg?token=5ac050c927682c56e1a83f226a547f00d4e4faee&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g014.jpg?token=e7501a5c573544840d40080d4e60598db9d4addf&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g015.jpg?token=db0af9327aa5f75197427efa4f079be70201cd13&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g016.jpg?token=fddda5e1fdaf1c456b386d2cde2e0ae95a3f58c0&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g017.jpg?token=d3d1d2571174e1d0e731e3dcfe1533786b66a7ed&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g018.jpg?token=3dc78deac0ca8c49de91153de8081c64a547d2b8&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g019.jpg?token=3b6fdbcbc5a635265d4c6e7ca6ee765928b679da&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g020.jpg?token=d5ca636870fd8cec3080a78552521409f30a5c42&ttl=1675958400","//zjcdn.mangafox.me/store/manga/21619/126.0/compressed/g021.jpg?token=4526d1cb5eebfb3fef3dc90f66a6369cc937a3aa&ttl=1675958400"];var newImginfos=[28427138,28427139,28427140,28427141,28427142,28427143,28427144,28427145,28427146,28427147,28427148,28427149,28427150,28427151,28427152,28427153,28427154,28427155,28427156,28427157,28427158,28427159];"#,
    )
}
