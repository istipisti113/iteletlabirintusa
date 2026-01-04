var ellensegek = []
var segedfile

var kapsebbonusz=0
var oksebbonusz=0
var szerencsebonusz=0
var lastdamage

function loadsegedfile(){
  fetch("/segedfile").then(resp => resp.json()).then(dat=>{
    segedfile=JSON.parse(dat)
  })
  console.log("segedfile betoltve")
  return
}
loadsegedfile()


function dobas(){
  return Math.random()*5+1
}

function szerencseproba(){
  luck-=1
  console.log(luck)
  var osszeg = dobas()+dobas()
  if (osszeg<=luck){
    return true
  }
  else {return false}
}

function korinditas(){
  var enemy_agil = Number(document.getElementById("eagility").innerHTML)
  var enemy_health = Number(document.getElementById("ehealth").innerHTML)
  var ellensegtamado = dobas()+dobas()+enemy_agil
  var sajattamado = dobas()+dobas()+agility

  if (sajattamado>ellensegtamado){
    lastdamage="player"
    enemy_health-=2
    document.getElementById("edamage").innerHTML=2
    document.getElementById("pdamage").innerHTML=""
  } else if (sajattamado<ellensegtamado){
    lastdamage="enemy"
    health-=2+kapsebbonusz
    document.getElementById("edamage").innerHTML=""
    document.getElementById("pdamage").innerHTML=2+kapsebbonusz
  }

  if (health<=0){
    halal()
  } if (enemy_health<=0){
    enemy_health=0
    ellensegek.splice(0,1)
    if (ellensegek.length==0){
      document.getElementById("bezaras").disabled=false
      document.getElementById("korinditas").disabled=true
      document.getElementById("szerencsehasznalat").disabled=true
      document.querySelectorAll("button.lapozasgomb").forEach(button=>{
        button.disabled = false
      })
    } else{
      harcablak(ellensegek[0])
    }
  }
  document.getElementById("ehealth").innerHTML=enemy_health
  document.getElementById("phealth").innerHTML=health
  document.getElementById("szerencsehasznalat").disabled=false
}

function csataszerencse(){
  console.log("csataszerencse")
  if (szerencseproba()){
    if (lastdamage=="player"){
      document.getElementById("ehealth").innerHTML=Number(document.getElementById("ehealth").innerHTML)-2
    } else{
      document.getElementById("phealth").innerHTML=Number(document.getElementById("phealth").innerHTML)+1
      health+=1
    }
  } else {
    if (lastdamage=="player"){
      document.getElementById("ehealth").innerHTML=Number(document.getElementById("ehealth").innerHTML)+1
    } else{
      document.getElementById("phealth").innerHTML=Number(document.getElementById("phealth").innerHTML)-1
      health+=1
    }
  }
  document.getElementById("luck").innerHTML=luck
  document.getElementById("szerencsehasznalat").disabled=true
  lastdamage=""
}

async function harc(enemies){
  if (enemies.indexOf(";")>-1){
    enemies = enemies.split(";")
  } else {
    var enemies = [enemies]
  }
  ellensegek = []
  for (var i = 0; i<enemies.length; i++){
    ellensegek.push(enemies[i].split(","))
    continue
  }
  harcablak(ellensegek[0])
}

async function harcablak(enemy){
  console.log("harcablak")
  //loadPage("/harc", enemy[0])
  loadPage("/harc", "harc")
    .then(_=>{
      document.getElementById("ehealth").innerHTML = enemy[2]
      document.getElementById("eagility").innerHTML = enemy[1]
      document.getElementById("ename").innerHTML = enemy[0]

      document.getElementById("phealth").innerHTML = health
      document.getElementById("pagility").innerHTML = agility
      document.getElementById("pname").innerHTML = "jatekos"
      document.getElementById("luck").innerHTML = luck
      document.getElementById("korinditas").onclick = korinditas
      document.getElementById("szerencsehasznalat").onclick = csataszerencse
    }).then(_=> document.getElementById("harcbutton").disabled=true)
  return Promise.resolve("asfd")
}

function halal(){
  console.log("meglaltal halo")
  window.location.href = "/deathscreen"
}

function loadCard(page, id, cardid) {
  fetch(page)
    .then(response => response.text())
    .then(data => document.getElementById(id).innerHTML = data)
    //.then(_ => loadsegedfile())
    .then(_ => newcard(cardid))
}

async function newcard(id){
  if (document.getElementById("harcbutton")!=null){
    document.querySelectorAll("button.lapozasgomb").forEach(button=>{
      button.disabled = true
    })
  }

  if (typeof segedfile[id] === 'undefined'){
    return
  }

  if (typeof segedfile[id].vane !== 'undefined'){
    document.getElementById(segedfile[id].irany.split("-")[0]).disabled = true
    document.getElementById(segedfile[id].irany.split("-")[1]).disabled = true
    //animacio vagy valami
    if (items[segedfile[id].vane] != undefined){
      document.getElementById(segedfile[id].irany.split("-")[0]).disabled = false
    } else {
      document.getElementById(segedfile[id].irany.split("-")[1]).disabled = false
    }
  }

  if (typeof segedfile[id].item !== 'undefined'){
    var itemek = segedfile[id].item.split(";")
    for (var i=0;i<itemek.length;i++){
      var splitted = itemek[i].split(" ")
      var op = splitted[0]
      var item = splitted[1]
      if (op[0]=='+'){
        if (typeof items[item] === 'undefined'){
          items[item]=0
        }
        items[item]=Number(op.slice(1))
      } else {
        if (typeof items[item] !== 'undefined'){
          items[item]-=Number(op.slice(1))
        }
      }
    }
  }


  if (typeof segedfile[id].kapsebbonusz !== "undefined"){
    kapsebbonusz=Number(segedfile[id].kapsebbonusz)
  } else{
    kapsebbonusz=0
  }

  if (typeof segedfile[id].eletero !== "undefined"){
    var adding
    if (segedfile[id].eletero.slice(1) === "d"){
      adding = dobas()
    } else {
      adding = Number(segedfile[id].eletero.slice(1))
    }
    if (segedfile[id].eletero[0]=="+"){
      health+=adding
    } else {
      health-=adding
    }
  }

  if (typeof segedfile[id].szerencse !== "undefined"){
    if (segedfile[id].szerencse == "proba"){
      document.getElementById(segedfile[id].irany.split("-")[0]).disabled = true
      document.getElementById(segedfile[id].irany.split("-")[1]).disabled = true
      //animacio vagy valami
      if (szerencseproba()){
        document.getElementById(segedfile[id].irany.split("-")[0]).disabled = false
      } else {
        document.getElementById(segedfile[id].irany.split("-")[1]).disabled = false
      }
    } else {
      var adding
      //if (segedfile[id].szerencse.slice(1) === "d"){
      //  adding = dobas()
      //} else {
      //}
      adding = Number(segedfile[id].szerencse.slice(1))
      if (segedfile[id].szerencse[0]=="+"){
        health+=adding
      } else {
        health-=adding
      }
    }
  }
}

