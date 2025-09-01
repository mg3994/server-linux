# 🚀 Multi-Vendor Delivery Server - Complete Documentation

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture & Design](#architecture--design)
3. [Getting Started](#getting-started)
4. [API Documentation](#api-documentation)
5. [Development Guide](#development-guide)
6. [Deployment Guide](#deployment-guide)
7. [Security & Best Practices](#security--best-practices)
8. [Performance & Monitoring](#performance--monitoring)
9. [Troubleshooting](#troubleshooting)
10. [Contributing](#contributing)

---

## 🎯 Project Overview

### What is Multi-Vendor Delivery Server?

A high-performance, production-ready HTTP/3 delivery platform built with Rust, designed to handle multi-vendor food delivery operations similar to Uber Eats, DoorDash, or Zomato. The system supports real-time order management, Firebase authentication, modern FCM push notifications, UPI-based payments, and comprehensive user management.

### 🌟 Key Features

- **🚀 HTTP/3 Support**: Built on Quinn QUIC protocol for maximum performance
- **🔐 Firebase Authentication**: Secure JWT verification with dual email/phone validation
- **📱 Modern FCM Integration**: OAuth 2.0 based push notifications (no deprecated server keys)
- **💳 UPI Payment Processing**: Google Pay integration with automatic refunds
- **📍 Real-time Location Tracking**: Live delivery tracking with WebSocket updates
- **🏪 Multi-Vendor Support**: Complete restaurant and delivery person management
- **⚡ High Performance**: Async Rust with Tokio runtime for maximum concurrency
- **🛡️ Production Ready**: Comprehensive error handling, logging, and monitoring

### 🏗️ System Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        Mobile[Mobile Apps]
        Web[Web Dashboard]
        Admin[Admin Panel]
    end
    
    subgraph "Load Balancer"
        LB[CloudFlare/Nginx]
    end
    
    subgraph "Application Layer"
        HTTP3[HTTP/3 Quinn Server]
        Auth[Firebase Auth Middleware]
        Router[Axum Router]
        
        subgraph "Core Services"
            Orders[Order Service]
            Users[User Service]
            Restaurants[Restaurant Service]
            Delivery[Delivery Service]
            Payments[Payment Service]
            Notifications[Notification Service]
        end
        
        subgraph "Real-time Layer"
            WS[WebSocket Manager]
            Location[Location Tracking]
            Chat[Communication Service]
        end
    end
    
    subgraph "Data Layer"
        DB[(PostgreSQL)]
   y 2024*d: Januar updateLast

*---m

.co-platformiveryport@delil**: sup **Ema
-ons)discussir/ery-server-delivlti-vendour-org/muhub.com/yottps://gitscussions](h[GitHub Dins**: iscussio**D
- sues)ry-server/isr-deliveendoi-v/multom/your-org.cs://githubttpssues](h [GitHub I**:
- **Issuestform.com)-plaerys.delivttps://doc.com](hy-platformcs.deliver/do[https:/tation**: **Documenort

- 

## 📞 Supp
---entation
/3 implemr HTTPt** fonn Projec**Qui
- rvicesn seatioor authenticm** fbase Tea **Firework
-mera f for the webtributors**m Cone
- **Axuc runtim the asynTeam** forio  **Tokem
-ng ecosystthe amazi** for unity**Rust Comm

- tsenknowledgmAc---

## 🙏 etails.

) file for dNSE](LICENSEthe [LICEnse - see MIT Licer the undesed cens lioject i

This prLicense--

## 📄 on

-roducti Deploy to psts
- [ ]oke terform sm [ ] Peent
-ing environmagsteploy to - [ ] Dmage
er iublish Dockld and pg
- [ ] Buiease ta] Create rel[ ion
- umentatate doc
- [ ] Updt suiteRun full tes[ ] 
- ANGELOG.md`date `CH Up
- [ ].toml` in `Cargoe version] Updatlist
- [ Checkse  Relea### 2.le)

#ard compatibkwxes (bacBug fi: H**ATC
- **Pible)ompatward ctures (back: New feaOR***MINs
- *change Breaking *MAJOR**:mVer):
- *ning (Sesiotic VerSemanollow ng
Fumberi. Version N## 1
## Process
# Releases

##quirement reicatione authent
- Providnd messageses ar codocument erro D
-xamplessponse ee request/reInclud
- cations specifiwagger OpenAPI/S- Use
ntationume API Doc

#### 2.
}
```tation Implemen    //<Order> {
Resultser) ->  user: &UderRequest,teOreaCrst: f, requeer(&seln create_ordasync f``
pub  `r.id);
/// {}", order:orde"Created intln!(
/// prr).await?;st, &userder(requee_oice.creat servorder =t
/// let ```rus
/// 
/// mples
/// # Exafails
/// essing ment procay/ * Pable
//ot avail is ntaurant The rescated
/// *ntiauthes not user i The f:
/// *ror ierrn an on will retutiis func 
/// ThErrors
/// 
/// # ///or.
 errer or and ordteeae cring thr>` containrdeult<Oa `Res/ Returns 
/// 
//rns# Retu
/// 
/// he order creating ticated userauthentThe  * `user` - tails
/// delivery deanditems ining uest contareqon  creatiert` - The orduesreq 
/// * `///ments
Argu/// 
/// # ant.
and restaurmer d custohe specifieder for t a new oreates
/// Crn
```rustcumentatio1. Code Do## rds

##on Standantati### Docume``


}
`atus_ok();e.assert_strespons
         .await;
       d"}))
: "confirmetatus"({"s.json(&json!))
        idorder.tatus", rs/{}/st!("/ordermafo  .put(&r
      nse = servet respotus
    lee sta// Updat    
    ;
.json()r = responseder: Ordelet or;
    created()atus_e.assert_stespons 
    r      .await;
  est())
   qu_reorder&create_     .json(  ")
 rders"/ot(       .posver
 ser= esponse let rr
    reate orde   // C
    p();
 nwra.uapp)new(er::stServ server = Te let   ;
().awaitppest_ap = create_t   let ap{
 low() pi_fer_a fn test_ordnctest]
asy

#[tokio::estServer;test::T
use axum_rsest.ntegration_t/ tests/i``rust
/n Tests
` Integratio#### 2.
```

    }
}::Placed);
OrderStatuss, (order.staturt_eq!       asse
 unwrap();t.rder = resul o        let
        
);ok()ult.is_es!(rert   ass     .await;
request)r(eate_ordeervice.crresult = s   let         
     };
    data
     test // ...           uest {
 rderReqeOest = Creat  let requ      ().await;
st_servicete_terearvice = c    let se{
    () ccessrder_sucreate_oc fn test_ asynst]
   #[tokio::te   
     est;
use tokio_t
    er::*; sups {
    usetestst)]
mod t
#[cfg(tes
```rusnit Test. U
#### 1lines
ing Guide### Test

inerstaom maint review fresass
5. Requ checks pe all CIn
4. Ensurationte documey
3. Updattionalitor new func tests f2. Writemain`
rom `ch fane feature br
1. Creatrocess Request P. Pull### 3
```

#ationument docnticationte authe): updadocs(api updates
in status case dgendle es): haix(orderication
fse JWT verifdd Firebaat(auth): afe*:
```
Examples*
**ore
, test, chctortyle, refaocs, sat, fix, d: fees**
**Typ
er]
```footl optiona

[dy]optional boon

[riptiope): desc```
type(scmat
orge Fommit Messa### 2. Cntation

#ve documerehensie compgs
- Writwarnin`  clippys all `cargo Addres
-attingor form` fe `cargo fmt Us
-inese guidel Rust stylow officiallle
- Fouidt Style G#### 1. Rusards

# Code Stand```

##tall
   inst ommi
   pre-ccommitre-stall p
   pip inmmitpre-constall    # Iash
`b ``**
  it Hooksup Pre-comm

3. **Set
   ```argo-watchated c-outdcargodit all cargo-austin   cargo 
denciesent depenelopmevtall dIns 
   # 
  fmtsty rudd clippomponent a
   rustup cee stablrustup updathain
   t toolcall Rus Insth
   #``basols**
   `t ToDevelopmenll 2. **Insta ```


  -serverrydor-delive-vend multigit
   crver.elivery-sevendor-di-ername/multr-usoucom/yub.https://gith clone  gitsh
  ba   ```*
k and Clone*
1. **ForSetup
ment evelop
### Dg
butin
## 🤝 Contri-

--))
}
```
o().intase(eDatab AppError::(|e|err.map_  result
        }
  ailed"),
Query f %e, " error =is(),tion.as_mill = durauration_ms(dror!racing::er => tr(e)     Er),
   ccessfully"cuted su"Query exe, as_millis()tion.on_ms = duraatinfo!(dur::i=> tracing   Ok(_) 
     sult {h &retc   
    mad();
 art.elapseion = stet durat l;
   l).awaitpoolf.(&secuteexey(query). sqlx::quert result =();
    letant::now::Insimed::trt = stt sta{
    let> yResulPgQuerult<) -> Res: &strlf, queryery(&see_quc fn executub asyn)]
plf)t(skip(senstrumen
#[i;
ResultPgQuerys::stgrese sqlx::post
ug
```ruy LogginQuerase  3. Datab
####
}
```
await  .(span)
  ent    .instrum }
   onse
        resp");
letedequest comp:info!("R  tracing:
      ait;(request).awun.re = nextspons relet    ");
    g request"Processing::info!(     tracin move {
       async    

);quest_idret_id = %st", reques("requen!o_spatracing::infn =  let spa();
   new_v4= Uuid::equest_id 
    let rse<Body> {-> ResponBody>,
) xt<   next: Nest<Body>,
 Reque  request: quests(
  race_re fn tub asyncn;

png::Spa
use tracid;id::Uuie uurust
using
```quest Trac 2. Re
####}
```
();
   .init)
     .json()ayer()t::lber::fmsubscrih(tracing_       .wit
    ))   ()),
  too".in|_| "infrap_or_else(.unw")"RUST_LOGnv::var(    std::e       :new(
 lter:vFiber::En_subscriwith(tracing
        .stry()ber::regiri_subsc  tracing) {
  g(t_logginnin i

pub fberInitExt};scrit, util::SububscriberExlayer::Sscriber::{ tracing_subuse`rust
ration
``ng Configu# 1. Loggiols

###ing To
### Debuggoverhead
ion erializatReduce saching
- plement c
- Imalgorithms efficient se morepaths
- Ue hot code timizOp:
- tions**``

**Solu
`erver--bin samegraph ph
cargo flmegraflanstall o icargcation
e appli Profile th``bash
#is**:
`iagnos

**DsageHigh CPU U

#### 3. ead replicasImplement ring
- poolnnection e co- Uspatterns
e query 
- Optimizindexesse - Add databations**:
``

**Solud_conf();
`oa_relpg;
SELECT 1000ement = tation_sg_min_duratEM SET loR SYST
ALTE';ment = 'alltate SET log_sER SYSTEM
ALT PostgreSQLogging iny luer Enable qsql
--is**:
```osgnes

**Dia QueriDatabase 2. Slow 
####r
atoocmalloc allEnable je
- esrge respons lag forminse streaers
- Uket handl WebSocleanup inroper ct p
- Implemenl sizesion pooconnectuce :
- Red*Solutions**>
```

*p -p <pidtover
grep serl
ps aux | erminaother tse
# In ann --releargo ru usage
canitor memory`bash
# Mois**:
``ose

**Diagn Memory Usag#### 1. HighIssues

ance rm### Perfo}
```

())
 Ok(  }
   wait?;
  _token().assccelf.refresh_a      se
  ) {w() >= exp:noc:exp| Ut(true, |s_at.map_oren_expireself.tokif ()> {
    t<) -> Resulelfoken(&mut ssure_valid_t enc fn
asyn refreshenc tokautomatilement ust
// Imp:
```rlution**
```

**Sothorized 401 Unauvery failed:cation deliNotifired
```
 token expisue**: OAuthures

**Isailion Fotificat 4. FCM N

####```ait?
aw
    .ase_url)databconnect(30))
    .s(::from_securationut(Dcquire_timeo    .ations(50)
 .max_connec::new()
   lOptionsPgPoong
andli h timeoutand adde pool size eas/ Incrt
/``rus*:
`**Solution*

```
ol exhaustedpon ioor: connectase err``
Databausted
`on pool exhti**: Connec
**Issues
suetion Isonnec C 3. Database``

####(())
}
` Okwait?;
   n().asosponse.j relic_keys =pub self.   ?;
    
.await()
        .send        nt.com")
erviceaccouem.gsken@systcuretose09/adata/x5obot/v1/metcom/rleapis.ogw.gops://wwget("htt       .lient
  self.ce =ns respo> {
    let-> Result<()mut self) eys(&ic_kpublesh_fn refrc ed
pub asynare refreshpublic keys re st
// Ensuon**:
```ruuti**Solre
```

 signatuokend tali failed: Invonnticati
Authe fails
```ificationen vere**: Toks

**Issucation Issuease Authentiireb
#### 2. F}
```
ection
dle conn
    // Hanonn.await?; cn =nnectio co letit {
   ept().awant.accpoi) = endt Some(conne lehilddr)?;
wt_ag, socke_confierer(servt::servoinEndp= ndpoint 
let est Quinn API latedate torust
// Up
```Solution**:
**``
nnecting`
` `quinn::Co`, foundcominguinn::Ind `qexpecteed types
tchmisma: error[E0308]ty
```
litibi API compa/H3sue**: Quinn

**Issrortion Erla### 1. Compi
#mon Issues

### Com
ngshootiublero-

## 🔧 T
```

--  ]
  }
}    }
   ]
    }
              
 s/sec"or"5xx err: endFormat" "leg           )",
5m]\"}[5..status=~\"total{requests_http_: "rate("expr"        {
           
   ": [   "targets",
     "graphype": 
        "tr Rate", "Erro":    "title
    {      
  },
          ]       }
  le"
   ntih perce"95tat": dForm"legen            )",
ket[5m])uc_seconds_btionuraest_dttp_requ, rate(h.95antile(0m_qu "histogra "expr":          {
      
     rgets": [ta       ",
  "graph"e":  "typ  
     Time",: "Responsetitle"
        "    {     },
  ]
 
           }"
       uests/sec"Reqmat": egendFor        "l  [5m])",
  al_totrequests(http_"rate": expr          "  {
          : [
targets"   "",
     aph: "gr   "type"",
     est Rate": "Requle       "tit {
      ": [
ls
    "pane",orm MetricsPlatfvery eli: "D"title"    ": {
ddashboar
  "`json
{ration

`` ConfiguhboardGrafana Das

### 
   ```)
   }    }ces,
      servi   ,
     to_string()ION").KG_VERSGO_Pnv!("CARversion: e      w(),
     Utc::nomestamp:      ti
      ),ng(rito_sthy".s: "healtstatu          {
  tatuslthSson(Hea    J 
     };
      
     g()),rin".to_sthealthyg(), "uns".to_strin"redit(ices.inserserv =>      Err(_)),
      ring()lthy".to_st "hearing(),stto_".s("redices.insert(_) => servi    Ok     {
   waitping().ais.e.redp_stat    match ap  eck Redis
  // Ch  
      
            };g()),
strino_".t"unhealthy_string(), e".toabas("datrt.inse> services_) =   Err(
        ring()),y".to_st"healthing(), to_str"database"..insert( => services    Ok(_) {
       ().await.db.ping_stateapp    match se
   abak datChec    //  
   ();
      hMap::newes = Hasmut servic      let 
 {> tatusthS> Json<HealState>) -: State<Appe(app_state)ck(Stat health_cheub async fn
   
   ping>,
   }<String, Strces: HashMappub servig,
       in Stron:si    pub ver   Utc>,
me<amp: DateTiest   pub timing,
    tatus: Str   pub s
    atus {HealthSt struct   pub]
 e)alizrive(Seri   #[de  ```rust
s**
  Check
3. **Health`
``}
   }
   
            }
       Err(e)       
       e order");reatto cd e, "Faile % =!(error       error {
           Err(e) =>}
        
           k(order)       O    ly");
    successfulted "Order crearder.id, %oid = nfo!(order_         i     => {
   Ok(order)          ait {
equest).awrder(rss_ocepromatch self.  
             );
     
 der"ating new or   "Cre    t_id,
    st.restauran = %reque_id restaurant    ,
      ser_idst.ureque = %_iduser   
        fo!({
       iner> lt<Ordst) -> ResuerRequeeOrd: Creatuestr(&self, reqate_ordec fn creub asyn  p(self))]
 t(skipnstrumen  #[i
   
 };, instrumentrorwarn, erinfo, g::{se tracin u```rust
     **
red Logging2. **Structu```


   }
   ration);.observe(duATIONEQUEST_DUR_RTP
       HTnc();.iS_TOTALSTHTTP_REQUE
        {ation: f64)uest(durecord_requb fn r}
   
   pp();
     ).unwras"
      in secondst duration"HTTP reque 
          s",ndcoration_se_durequesthttp_       "::new(
    stogramtogram = HiN: HisURATIO_REQUEST_D ref HTTP static     
       unwrap();
 
       ).quests"HTTP reer of numb  "Total    ,
      ests_total"equttp_r"h        w(
   Counter::ne= Counter TAL: UESTS_TOf HTTP_REQc re       statiic! {
y_stat   
   lazistry};
 RegHistogram,{Counter, rometheus::e pt
   us   ```ruscs**
s Metrimetheu **Protup

1.itoring Se`

### Mon
   ``())
   }k(
       O
        }
                }", e);
 : {}tionicad notifiled to senwarn!("Faacing::    tr         lt {
  ) = resur(ef let Er   i       {
  ultsresr result in  
       fo      wait;
futures).ae::join_all(turures::fults = fut  let resu           
 d));
ken, &payloation(&to_notifica.send)| selfen, payload  .map(|(tok        
 into_iter()        .
   cationss = notifilet future
       ult<()> {-> Resd)>,
   ) yloaionPa, Notificatringns: Vec<(Stotificatio nlf,
         &mut se  cations(
  notifiend_bulk_ sb async fnrust
   pu   ```g**
intch Process**Async Ba``

3.    `  }
   }
e),
     Nonk(   None => O       ?)),
 r(&data)m_stfron::e(serde_jsoOk(Somme(data) =>         So  hed {
 h cac  matc   
       ;
  key).await?.redis.get(& = selftion<String>t cached: Op      leer_id);
 , us"user:{}"at!( key = form  letr>> {
     Useult<Option<esid) -> Rer_id: Uuus&mut self, cached_user(et_nc fn g
   pub asy  ands;
 s::AsyncComme redi   usust
``rng**
   `CachiRedis 

2. **`  }
   ``to()))
 atabase(e.in::D AppErrorp_err(|e|     .maait
         .aw      )
  abase_url.connect(dat       800))
    :from_secs(1n:ime(Duratiox_lifet   .ma  ))
      cs(600ion::from_semeout(Durat.idle_ti      
     _secs(30))fromDuration::meout(acquire_ti         .ons(5)
  _connecti  .min
         ons(20)onnectiax_c    .m   ()
    :newolOptions:    PgPoPool> {
   lt<Pgtr) -> Resu&stabase_url: e_db_pool(dacreatnc fn    pub asyions;
   
oolOpt:PgPres:lx::postgsquse ```rust
   ing**
   Poolection nntabase Co*Da

1. *tionce Optimiza## Performanng

#tori& Monierformance  P

## 📊``

---    ))
}
`lock"),
"1; mode=bstatic(om_e::frHeaderValu,
        ")ction"x-xss-protem_static(erName::froHeadg(
        ::overridinerLayersponseHeadetRe(S.and
    ))Y"),
    "DEN_static(alue::from  HeaderVS,
      _OPTIONAMEX_FRader::he
        overriding(derLayer::ResponseHea    .and(Set    )
),
niff"tatic("nos_slue::from   HeaderVaNS,
     _TYPE_OPTIONTONTEheader::X_C
        verriding(ayer::oHeaderLsespon
    SetReue> {erValer<HeadseHeaderLayetRespon) -> S_headers(rityfn secub 

pu;eaderLayertResponseHt_header::Setp::sese tower_ht
```rust
uders
ity HeacurSe`

### 
}
``e
    )per minuts(60), // om_secfration::      Durests
  0, // requ   10(
     ewyer::nRateLimitLa   ayer {
 mitLRateLi() -> iddlewareimit_mte_rate_lrea
pub fn cDuration;
std::time::e 
useLimitLayer;it::Ratlime tower::t
us`rus
``ng
te Limiti
### Ra
   ```

   }ields f.. other// .     
  one: String,     pub phg)]
  zinlie(skip_seria    #[serdg,
   mail: Strin      pub eg)]
 alizine(skip_serierd
       #[sub id: Uuid,
       puct User {  pub str
 Debug)]   #[derive( logs
ta insitive daMask sent
   // 
   ```rusing**Handl**PII   ```

2. 
   }
 o()))(e.intInternalrror::(|e| AppEap_err  .m         )
es().as_bytonce, datat(ncrypr.enciphe
       
       nce());te_noneraslice(&gece::from_nce = Nonno       let ;
ey))(kce::from_slicm::new(Key6Gpher = Aes25      let ci<u8>> {
 lt<Vec8]) -> Resur, key: &[udata: &stitive_data(encrypt_sensn pub f  
   
 ce}; Key, Noncm,es256Gs_gcm::{A aet
   use  ```rus*
  at Rest***Encryption

1. Protection

### Data 
   ``` }k(row)
    O   
     
    .await?;       )
lf.poolch_one(&se  .fet        )
    
_id      order
     ",1 = $ERE ids WHorderOM "SELECT * FR            Order,
         (
 x::query_as! = sqlt row    le{
   er> > Result<Ord: Uuid) -, order_idorder(&selfn get_c fb asyn   pu
zed queriesmeteriys use para   // Alwa  ```rust
vention**
 ction PreSQL Inje`

2. **``
   }
   dress,ss: Adery_addrepub deliv    
   alidate] #[v       
  
    rItem>,Orde items: Vec< pub))]
      ed"m requirst one itee = "At leaagin = 1, messh(mlengtte(da     #[vali    
  ,
   id: Stringrant_tauub res]
       ped")) ID requir"Restaurant=  1, message n =gth(milenidate(val  #[    Request {
 ateOrderCretruct 
   pub s Validate)]ize,erial Desebug,rive(D   
   #[de;
ionError}at, Valid{Validatetor::dase vali
   ust ```rudation**
  uest Vali**Req
1. dation
t Vali

### Inpu}
   ```
       Ok(())}
   ;
       into()))ed".ot verifie nn("Phonhenticatiorror::Autrn Err(AppEtu  re         
 {ifiede_verer.phonus    if !      }
   nto()));
 verified".iail not ion("EmicatentthAppError::Aurn Err(etu r
          d {l_verifieemair.if !use     {
   lt<()>er) -> ResuUsuser: &self, s(&rementequir_ruse validate_pub fnt
   ```rus   ts**
quiremenion Reatrific Ve. **Dual
2

   ```  }laims)
 ta.cs(&token_dae_claimself.validat   n)?;
    &validatioblic_key, oken, &pulaims>(t::<C decodeen_data = toket l 
          256);
  ithm::RSAlgornew(tion::n = Validavalidatio let t?;
      d).awaiic_key(&kipublelf.get_ic_key = s   let publ       
)?;
    ".into())Dy Iing ke"Misshentication(pError::Autkid.ok_or(Ap = header.    let kid
   n)?;(toke_header= decodeet header 
       lUser> {ult<r) -> Resen: &stf, tokseltoken(&erify_b async fn vtures
   puJWT signaways verify   // Alst
 
   ```ruidation** JWT Val*Firebase
1. *
urityn Sectio# Authenticactices

##t Praurity & Bes
## 🛡️ Sec--
nted

-s documerocedurevery p] Reco
  - [ etcies sention poliet[ ] Data rd
  - n teste placoverysaster re [ ] Diated
  -s autom backupDatabase - [ ] ry**
 ecove*Backup & R ] *ets

- [c assatisetup for stDN d
  - [ ] Cnfigure] Caching coized
  - [ se optimbata] Da
  - [ tedng completiad tes] Lo- [ rmance**
  ] **Perfod

- [ reiguegation conf ] Log aggrup
  - [s set lerting rule  - [ ] Ared
 configurdsashboarafana d [ ] G
  -abledtrics enmetheus me  - [ ] Proitoring**
- [ ] **Monpleted

ng comity scanniecur - [ ] Sions set
 rmiss RBAC pe- [ ]ured
   configieswork polic  - [ ] Netnaged
perly maets pro ] Secr- [y**
  ] **Securit- [ talled

ates insificrt [ ] SSL ces
  -dentialeway crePI gat[ ] U
  - figuredject con proFirebase- [ ] r setup
  Redis cluste[ ] red
  - e configuasatabroduction d
  - [ ] Ption**nt Configuravironme*Ent

- [ ] *ecklisyment Chon Deploducti
### Pro
 80
```     number:       port:
        
      ver-servicevery-ser: deli        name
    ervice:
          send:       backx
 ype: Prefi    pathT   h: /
 at- pths:
      :
      pa    httptform.com
lalivery-pst: api.des:
  - horule
  r-tlsivery-serveetName: delecr
    sm.comforivery-plat - api.dels:
    host:
  -tls
  
spec:"ct: "true-redireo/ssls.ikuberneteess.ngr nginx.it-prod
   ncrypssuer: letsester-io/clu-manager.i
    certginxs: nngress.clases.io/i  kubernetations:
  m
  annotlatfor-plivery: despace
  namer-ingressery-servedelivme: 
  naata:gress
metadv1
kind: Inio/ng.k8s.orkitwion: ne--
apiVerssterIP

-: Clupe8443
  tygetPort: 80
    tarport: 
  - 
  ports:rvery-servep: delir:
    apelecto:
  s
specy-platformver deliamespace:ice
  nr-servservery-: delive name
 metadata:ervice
v1
kind: SpiVersion: 
```yaml
a and Ingressvice
#### 4. Ser`
ds: 5
``riodSecon  pe
        Seconds: 5ialDelay     init443
     : 8ort    p        th
 /heal path:         pGet:
      htt:
      ssProbeadine
        reeconds: 10riodS    pe   
   : 30aySecondsel    initialD8443
      rt:     poth
         path: /heal
            httpGet:
         ssProbe:livene"
        00m    cpu: "5       "
 Miry: "512emo          mlimits:
      
       "250m"       cpu:   "256Mi"
    memory:        quests:
           rerces:
  esou
        rer-secretsame: serv        n    retRef:
     - sec
   configme: server-      na:
      efpRconfigMa   -     envFrom:
     
    443erPort: 8inta      - con  ports:
     atest
   server:lvery-image: deli   
     name: server:
      -  containers
         spec:-server
eryeliv      app: dabels:
  
      lmetadata:  ate:
   templr
 rvelivery-sepp: de   a  ls:
 be   matchLaor:
 lect 3
  seeplicas:ec:
  rform
spatery-pl delivnamespace:y-server
  deliverme: a:
  naatnt
metadDeploymev1
kind: n: apps/rsiopiVe
```yaml
aoymentpl# 3. De
```

###et>ded-secr<base64-encoRET:   JWT_SEC>
eyded-k<base64-encoI_API_KEY:   UPcoded-key>
base64-enUNT_KEY: <ACCOE_SERVICE_
  FIREBASl>ured-odencL: <base64-DATABASE_UR
data:
  paqueype: Otform
ty-plaace: deliver namespcrets
 er-seame: serv:
  nata
metad: Secret
kindon: v1piVersi
```yaml
a 2. Secrets
####
"
```IONS: "10X_CONNECT
  REDIS_MA"20": TIONS_MAX_CONNECSEABA"
  DAT: "infoLOGUST_8443"
  RVER_PORT: ".0"
  SERST: "0.0.0  SERVER_HOrm
data:
ivery-platfoace: del namesp-config
 : serverame ndata:
 Map
metaind: Configv1
kn: Versio-
apitform

---plaeryeliv  name: da:
tadat
meespacekind: Namv1
: piVersion
ayamligMap
``` Confespace and#### 1. Nament

etes Deploymubern

### Kata:
```
  redis_des_data:s:
  postgrlumever

vo     - ser
 epends_on:o
    d/nginx/ssl:rtc  - ./ssl:/e    conf:ro
inx.inx/ngf:/etc/ng ./nginx.cones:
      -"
    volum443:443     - "0"
     - "80:8
  rts:    poinx:alpine
mage: ng
    iginx:data

  n:/dis_data
      - rees:"
    volum6379:6379
      - "rts:
    popineredis:7-al: 
    image redis:"

 432  - "5432:5orts:
    ta
    pdaresql//postgata:/var/libres_d- postg
      volumes:    assword
SSWORD=pS_PA  - POSTGREgres
    _USER=postSTGRES   - PO
   tformy_plaDB=deliver- POSTGRES_      :
entironmnv   estgres:15
 age: poimdb:
    g:ro

  pp/confi/config:/a    - .es:
  um  voldis
   - re db
     
      -ds_on:    depen_LOG=info
ST - RU379
     edis:6redis://rL=IS_UR  - REDtform
    pla32/delivery_rd@db:54s:passwo://postgresql=postgreBASE_URLDATA
      - ment:nviron"
    e8443:8443  - "      ports:
 ild: .
   bu server:
  s:
 service

3.8'on: 'ml
versi``yapment
`for Develo Compose . Docker 2```

####erver"]
CMD ["s exit 1

lth ||43/healhost:84p://locahttrl -f  cuCMD3 \
    es=s --retriiod=5-perstart=3s ---timeout=30s -intervalLTHCHECK --443
HEA 8

EXPOSEER appusert user
US non-roo toitch

# Swn/serverr/local/bius / +xUN chmodserver
Ral/bin//locsrver /userelease/get/r /app/tarderuilm=bfroY --y
COPCopy binar
# se appuser
-s /bin/faleradd -r  us
RUNoot user non-rCreates/*

# listpt/lib/arm -rf /var/& 
    &bpq5 \    li3 \
    libsslates \
-certific\
    cal -y talget inste && apt-pt-get upda
RUN anciesme dependetitall runlim

# Inskworm-sbooROM debian:tage
F# Runtime s

in servere --bild --releaso bug)
RUN carginter cachetor birst (fs fncie depende

# Buildcsrc ./sr./
COPY k .locrgol CaPY Cargo.tomapp
CO
WORKDIR /
lder:1.75 as bui
FROM rustBuild stage# 
ileerfdock``
`lefiockeri-stage D. Mult#### 1loyment

epr DDockeide

###  Guployment De

## 🚀```

---}
REATED);
tusCode::Cus(), Sta.stat!(responsert_eq
    asse ();
   unwrap  .   
   await
        .   )(),
     apnwr   .u         
    }"#))st":"te{"name"::from(r#"dy(Body .bo            
   est-token"), "Bearer torization"r("auth     .heade      
     json")ication/, "applnt-type"der("conte   .hea         
    tems")i("/i      .ur         ")
 "POST(hodet          .m
      ilder():buuest:        Req
    hot(ones
        .e = app let respons;
   aitapp().awate_test_et app = cre    litem() {
t_create_ test]
async fn#[tokio::tesper::*;

suuse st.rs
_feature_tes/newestst
// tests
```ru T4. Write```

#### tem))
}
elete_i::dersdelete(handl", items/:id.route("/       te_item))
 dlers::updaut(han/:id", pe("/items.rout
        _item))::getandlersd", get(h/:ie("/items     .routem))
   te_iteacrandlers::ost(h, p"/items"route( .      r::new()
 teour {
    RRoutes() -> e_routenew_featurub fn 

pete}};, delputost, et, p::{gr, routingum::{Routeaxe 
usroutes.rs src/rust
//```
 Routes# 3. Add

###`tion
}
``mplementadler iHan
    // onse>> {Resp<Itemt<Json> Resuler>,
) -ension<UsExtser): ion(u  Extens
  h<Uuid>,th(id): Pat    Pat_item(
nc fn geub asy}

pmentation
lempdler iHan
    // sponse>> {Retem<Json<I
) -> Resultst>,ue<CreateReqJsonest): n(requr>,
    Jsonsion<Useer): Extesion(us   Exten
 te_item(eac fn cr
pub asyn;
rror::Resultcrate::eser;
use els::Uauth::moduse crate::::Uuid;
id
use uutension}; Exath,:{Json, P
use axum:lers.rsure/hand/new_feat src
//```rusts
API Handler. Create #### 2}
```

er methods
oth   // ...  
  }
   ation
   lementmp    // I{
    ult<Item> Res-> t) ques CreateRe, request:elfe_item(&sn creatsync f   apl {
 reServiceImwFeatuvice for NeFeatureSer]
impl Newaittr

#[async_n>,
}nnectiobaseCodyn Data db: Arc<Impl {
   eServiceurNewFeatstruct 

pub )>;
}<( -> Result Uuid)elf, id:e_item(&sn deletasync f  t<Item>;
  t) -> ResuldateRequesdates: Upid: Uuid, up&self, tem(ate_isync fn updem>;
    a<It -> Resultd): Uuiidtem(&self, c fn get_iasyn   tem>;
 esult<Iuest) -> Rt: CreateReqes(&self, reque_itematsync fn cre{
    ac Send + Syn: ureServiceatit NewFepub tra]
rait
#[async_tsult;
::error::Recrate;
use uid::Uide uu;
usc_traitasynt::sync_trai ae.rs
use/servicnew_feature/ src/rust
/
```yerrvice Lareate Se C

#### 1.New Features## Adding `

#d
   `` outdatergo  cak
 dency chec  # Depen
   
 auditt
   cargo urity audi # Sec
   
  check--o fmt  cargtting
  # Forma
   rnings
   -D waippy -- go cling
   car  # Lint ```bash
 cks**
  Quality Che. **Code 
3``
ity
   `securt --test tesrgo ests
   cay t # Securit   
  
aseeleest --rt load_tt --teses
   cargo t# Load tests      
gration
-test intecargo test -  ts
 on tesntegrati  # I
  --lib
     cargo testts
 t tesni# U
    ```bash
  *Strategy*. **Testing    ```

2re
atuw-feeature/nesh origin f
   git pueature" f: add newm "featit commit - gpush
  nd # Commit amt
   
   o f   carglippy
  cargo cty
 alieck code qu Cht
   
   #  cargo tesn tests
 
   # Ru
   tionenta documpdate# Uests
   te t Wrianges
   #   # Make chature
   
ture/new-fefeaheckout -b 
   git cture brancheate fea  # Cr``bash
    `pment**
Develoature w

1. **Feent Workfloopm# Devel##es
```

ion utiliti   # Locat  tion.rs     loca──ies
    └n utilitptiory    # Encyption.rs   ── encrion
    ├ut validat     # Inpon.rs  ├── validati  mod.rs
  ── ils/
    ├dels
└── ut moabase      # Dat  dels.rs    └── mo
│  ationsSQL migr#   s/       ationigr── mction
│   ├ conneabase      # Datction.rs  conne├──rs
│     ├── mod.│ abase/
─ datgement
├─ry mana   # Delivece.rs         └── servi models
│ talivery da       # De  rs   ├── models.
│  PI handlersery A Deliv       #.rs  ndlers│   ├── haod.rs
   ├── m/
│─ deliverynagement
├─ant mastaur # Re       vice.rs   ser└──   a models
│urant dat Resta         # models.rs    ├──handlers
│ rant API Restau# s         andlers.r
│   ├── h  ├── mod.rsrants/
│ 
├── restauentr managem      # Use.rs    ─ service└─  
│ a modelsdat# User        rs    s.  ├── models
│ handler # User API         rs.rsandle─ h.rs
│   ├─   ├── mod── users/
│on models
├tificati      # Nos     s.r└── modelice
│   serv     # FCM s         fcm.r│   ├── od.rs
── mons/
│   ├otificatiing
├── nprocessayment   # P       service.rs 
│   └──  data models   # Payment      dels.rs  ─ moers
│   ├─handlt API en # Payms         handlers.r
│   ├── mod.rs  ├──yments/
│ ogic
├── painess lbus Order    #   rs    ice.│   └── servs
elta modrder da         # Oodels.rs  ── mers
│   ├er API handlrd  # O      andlers.rs ── h.rs
│   ├od ├── mers/
│  ── ord
├modelsata   # Auth d  s.rs        model   └──eware
│dl # Auth mid     leware.rs middon
│   ├── tiauthenticaase  Fireb         #base.rs├── firemod.rs
│   ──    ├uth/
│── ahandling
├ types and ror        # Er   s     .r─ errorleware
├─ustom midd   # C       rs  middleware.ions
├──definitoute      # R       outes.rs   ation
├── rplement3 server imHTTP/   #            erver.rs ├── snagement
on mafiguration      # C    g.rs     fi con
├──ary root  # Libr           b.rs     
├── litinn entry po Applicatio #              n.rs  /
├── mai

```
srcctureStru# Project uide

##velopment G

## 💻 De``

---};
`ak;
  }
;
      breta.message), message.dation:'w notificaog('Neole.l    consion':
  e 'notificataseak;
    c
      bration);age.data.loc, messcation:'ivery lolog('Del    console.ate':
  updlocation_ase 'k;
    c      breas);
.data.statussagetatus:', meg('Order sole.lo      cons
s_update':tatuorder_s  case '  ge.type) {
witch (messaa);
  
  sent.datSON.parse(evage = Jnst mess> {
  cont) =e = (eve.onmessagvascript
wsjates
```Updaer time Ord### Real-```

#  }));
};
'
-tokenbase-jwt: 'firekento
    icate',ntauthe   type: 'y({
 if.stringsend(JSONtion
  ws.caend authenti S
  // { = () =>penno;
ws.ocom/ws')mple..exas://apiket('wsebSoc Wws = newipt
const ```javascrn
nnectio# Co
###ket API
 WebSoc``

###
}
`": 25.5  "speed": 45.0,
ding0,
  "hea: 5.cy" "accura7,
 ": 72.877dengitulo  "760,
: 19.0tude"
  "latioken>

{ Bearer <thorization:/json
Aut applicationntent-Type:
Coionsons/locatery-perPUT /deliv
```http
n Locatio## Update
###}
```
Doe"
  }
 "John er_name":holdt_    "accoun1234",
DFC000"H_code": fsc  "i7890",
  3456": "12rnumbe  "account_ils": {
   "bank_deta },
 "
 ument_urlce": "doc   "insuran
 nt_url",me": "docutionstrae_regi  "vehicl,
  ment_url""docucense": _liiving {
    "drments":"docu 1234",
 : "MH12ABber"e_num "licens
 cle", "motorcyicle_type":veh
{
  "<token>
ion: Bearer zatri
Authoation/jsonapplic-Type: s
Contentivery-personOST /del
```http
Py Personver Deliegistert

##### RagemenMan# Delivery ``

###
`
}  }
  ]     ]
        }
 ian"]
  etarveg"non-[o": nfietary_i "d
         time": 15,ation_ "prepar         true,
 ble":  "availa",
        /...ps:/: "httrl"   "image_u    00,
   0.": 28price   "     ry",
  o-based cur tomatCreamy": "escription     "dn",
     ckeutter Chi"name": "B                 {
 ": [
"items",
      Main Coursename": "  "    {
    : [
ories""categ{
  

oken>er <tBeartion: izahoron
Aut/jsicationt-Type: applntenCou
t_id}/menurans/{restaantaurPUT /rest
```http
Menu Update ```

#####}
}

  e.com"ant@exampl"restaur "email": 
   ",1-9876543210+9": "ne    "pho {
contact": },
  "0"}
 se": "22:000", "clo": "09:: {"openay"uesd},
    "t2:00"e": "2", "clos": "09:00{"open"monday": 
    hours": {ng_eratiop  ",
dian"]North Inn", ": ["Indias"isine_type
  "cu {...},ress":",
  "addn cuisineic India"Authentiption": 
  "descrites",licious Bame": "De
{
  "noken>
 Bearer <thorization:on/json
Autcatiliype: appntent-TCourants
estaT /rp
POS
```httstaurantgister Re## Reement

### Manag# Restaurant
###}
```
}
"]
  getarian"veictions": [restrtary_"diee"],
    nes, "Chin""Indias": [sine_typeui
    "c {es":"preferenc,
  00001"
  }e": "4"postal_cod",
    atrash"Mahar: state"   "Mumbai",
 "ty": 
    "ciMain St",t": "123 ree  "st
  ": {ddress"a10",
  91-98765432"+": 
  "phonehn Doe",Joe": "{
  "namoken>

 Bearer <thorization:
Autnion/jso: applicat-Type
Contentlers/profise
PUT /uttpofile
```he User Prpdat
##### U
oken>
``` <tearerorization: Bfile
Authers/pro
GET /us`httpProfile
``et User #### Gnt

#agemer Man
#### Useen>
```
arer <tokization: Be}
Authorayment_id{ps/ET /payment
G
```httpment Statust Pay#### Ge}
```

#:15:00Z"
1024-01-15Tt": "20es_apir  "ex?...",
"upi://pay": l"payment_ur
  9",2345678": "TXN1action_id
  "transing",ocess "pr"status":uuid",
  ": " "id{
 n
:**
```jsosespon
```

**Re: 450.00
}"amount"},
  "
  tmuser@pay: "
    "vpa"",: "UPI"type"    ": {
nt_method
  "paymed",uuid": "r_i
{
  "orde>
enok Bearer <tn:izatiojson
Authorion/pplicatt-Type: a
Conten /paymentsp
POST``httPayment
`# Process t

####nt Managemen### Payme}
```

#me": 20
paration_tid_pretimated",
  "es"confirme": us "stat
{
 n>
oken: Bearer <torizatiojson
Authtion/pplicaype: a-Tntents
Cotud}/stars/{order_iUT /ordettp
PStatus
```hate Order 

##### Upd }
}
```25:00Z"
 -01-15T11:"2024": val_arried  "estimat
  
    },: 72.8777itude""long,
      9.0760e": 1titud"la    ": {
  ion_locatrrent {
    "cung":acki,
  "tr00Z"5T11:30:01-1e": "2024-ivery_timimated_del
  "est0:00Z",0:01-15T1"2024-0": eated_atcr  "": {...},
ssaurant_addre},
  "rest: {...address"ry_
  "delive 450.00,al_amount":  "tot.],
items": [..sit",
  "": "in_transtatus  "uid",
n_id": "uy_persoer"delivuid",
   "u":staurant_idre,
  ""uuid"tomer_id": cus
  "d",d": "uuin
{
  "ijsoonse:**
```**Respn>
```

Bearer <toketion: izaid}
Authororder_ /orders/{`http
GETs
``r Detailrde# Get O##
```

##ing"
} "pendus":ayment_stat,
  "p456""DEL123": cking_idtra,
  "T11:30:00Z"2024-01-15": "ry_timeted_delive "estima
 0,.0": 450ount_am
  "totald",place "us":,
  "stat": "uuid""id
{
  ne:**
```jso*Respons
*
}
```
paytm"
  }"user@pa":     "v: "UPI",
 "type"": {
   nt_method"payme77
  },
  ": 72.87ongitude"l0760,
    itude": 19.
    "lat: "India", "country"0001",
   code": "40al_    "post
a",trshra"Maha"state": ,
    i""Mumba: ty"  "ci",
  ain St: "123 M "street": {
   y_address"eriv],
  "del }
  cy"
   Extra spiions": "structecial_in      "sp: 2,
uantity""q
      ",uuid: ""enu_item_id     "m [
    {
 ems":"itid",
  _id": "uu"restaurant
  

{token> Bearer <orization:/json
Authplicationent-Type: aprders
Cont/oPOST `http
``te Order
##### Creant

anagemeer M
#### Ord
```

  }
}healthy"rebase": "    "fihealthy",
s": "dire
    "althy","hee":    "databas
 ces": {"servi
  .0",ion": "1.0",
  "vers00Z10:30:4-01-15T202mp": "
  "timesta",hyhealtstatus": "
  "
{**
```json*Response:
```

*T /healthp
GEheck
```httalth Cs

#### He Endpoint Corepoint

###r the ende fopriate rolapproust have User mified
- t be vernumber mus
- Phone erifiedst be v- Email mus:**
irement**User Requ

```
jwt-token><firebase-arer ation: Berizp
Autho*
```htteader:* Hcation**Authentiication.

nthese JWT aute Firebah`) requiralt/hets (except `inll API endpo
Ation
ca## Authentiation

#I Document
## 📚 AP
---
=30
```
VALNTERK_I
HEALTH_CHECRT=9090METRICS_POg

# Monitorintion-key
your-encrypRYPTION_KEY=ret-key
ENCour-jwt-secRET=y_SEC
JWTurityt

# Sectorage-bucker-sET=youRAGE_BUCKey
STO-maps-api-kour-googleI_KEY=yces
MAPS_APal Servirn
# Exteapi-key
I_KEY=your-
UPI_APt-idmerchanANT_ID=your-m
UPI_MERCHcoider..upi-prov://apittpsAY_URL=hUPI_GATEWtion
onfigurayment C UPI Pason

#unt.j-accoceto/servith/T_KEY=paVICE_ACCOUNE_SERAS
FIREBroject-idfirebase-pCT_ID=your-JEBASE_PRO
FIREionConfiguratse eba
# FirIONS=10
CTIS_MAX_CONNE
RED79host:63//local=redis:
REDIS_URLnfigurationedis Co

# R0IONS=1NNECTASE_MAX_COform
DATABry_platt/delivecalhossword@loername:pas/usresql:/L=postg
DATABASE_URguration Confiatabase

# D=infoOG
RUST_LPORT=8443
SERVER_ST=127.0.0.1n
SERVER_HOuratioer Configenv
# Serv```tion:

urang configollowi with the f`.env` filereate a on

Cigurati Confnvironment## E

# ```.."}
  :"."timestamp"ealthy",:"hstatus"rn: {" retu # Should  /health
alhost:8443/loc:/url http
   c
   ```bashtion**allaerify Inst **V   ```

6.run
 cargo bash
   ```Server**
  velopment  De **Start   ```

5.n migrate
--bin o ruons
   cargrati # Run mig
  tform
   ivery_plaedb del creatase
  atabL dstgreSQreate Po# Cbash
      ```*
abase***Setup Dat  ```

4. tion
 raonfigur cth youit .env wi  # Ednv
 e .eamplnv.ex
   cp .e   ```bashronment**
*Setup Envi`

3. *  ``uild
 
   cargo bash``b   `ies**
ll Dependencta*Ins2. *   ```

y-server
eror-delivi-vend
   cd mult.gitserverr-delivery-multi-vendom/your-org/co//github.ps:one htt
   git cl   ```bashy**
 Repositorlone thert

1. **Cuick Sta

### Qt processingaymen pcount**: Fory AcPI Gatewa**Uled
-  FCM enabtion andicant Authe: Withect**ojebase Pr
- **Firlater 7.0 or *:**Redis*r later
- *: 15.0 oPostgreSQL*
- **ater.0 or l: 1.75**
- **Rustuisites
ereq Pred

###tting Start
## 🚀 Ge`

---

``tion>,
}ConnecseDataba: Arc<dyn  pub db  >,
 e>ervic<Mutex<FCMS: Arcon_serviceb notificatie>,
    puentServic<dyn PaymArcervice: t_s paymene>,
    pubrServicc<dyn Usece: Arervi user_s    pubice>,
OrderServdyn  Arc<vice:der_serpub or
     AppState {uct
pub stron patterntindency injecpe

// De
}Order>>;c<ult<Veters) -> ResderFils: Orter &User, filuser:lf, s(&se_orderlist   async fn t<Order>;
 er) -> Resul, user: &Usidrder_id: Uuelf, order(&snc fn get_o    asyt<Order>;
 Resulatus) ->erSt Ord, status:ider_id: Uu(&self, ordtusdate_stasync fn upder>;
    a<Or-> Result &User) er:Request, usteOrdert: Crearequesr(&self, rdecreate_o async fn Sync {
   : Send + Servicerder trait Oit]
pubync_traern
#[as pattervice traite sust
// Cor`rture

``rchitec# Service Aing

##tracand  metrics, ed logging,**: Detailervability**📊 Obsls
6.  all levesting atensive teCompreh: tability**. **🧪 Tes
5ncernstion of coeparaar swith clerchitecture lean ay**: Clitntainabi4. **🔧 Maiess design
th statelt wing supporontal scali**: HorizScalability
3. **📈 andlingd error h, ant validation, inpunticationthecure au**: Seby Defaultrity ️ Secuble
2. **🛡 possiopy whereth zero-cign wic-first dest**: Asynrsormance Fi**🚀 Perf

1. n Principlesig Core Desork |

###framewc testing synt | A | LatesstTokio-teesting** |  |
| **TbservabilityStructured og | 0.1+ | racin* | Togging***L| 
ng |andliON/data h+ | JS1.0 Serde | tion** |Serializa
| **ssing |ent procest | Paymation | Latentegr IUPIts** | ymen |
| **Paging serviceh messa| Pus| Latest h Aut** | FCM Oificationsn |
| **Nottioen verificaJWT tok | Latest | base Auth | Firetion**uthentica |
| **Ang cachin and data| Sessio7+  | Redis | 
| **Cache** storage |Primary data15+ | L | stgreSQ Pobase** | **Data|
|e nd middlewaruting a+ | HTTP roxum | 0.7mework** | A FraWeb**ol |
| ocQUIC prott | HTTP/3 Latesn + H3 | * | QuinTP Server*
| **HTion engine |ecutAsync ex| | 1.35+ o me** | Tokiunti--|
| **R----------------|-----|---|-------
|--------Purpose || sion Verlogy | hno | Tec
| Component Stack
ologyTechn## 
#ign
cture & DesArchite 🏗️ ##

---


```diss --> Re  User  --> Redis
Orders     
    s
tion --> Map  LocaPI
  ents --> UM
    Paym> FC --ionsat
    NotificerebasAuth --> Fi 
    
   --> ChatWS     ation
--> Loc 
    WS DB
   ments --> ay  P-> DB
  livery - DB
    Dets -->stauran> DB
    Re  Users --
  --> DBrs    Orde    
  WS
uter -->Rotions
     Notifica  Router -->yments
  -> Par -oute
    R> DeliveryRouter --    estaurants
 Ruter -->    Ro> Users
Router --Orders
    outer -->  R
    
   outer--> R
    Auth 3 --> Auth HTTP  -> HTTP3
 
    LB -  
   LBmin -->LB
    Ad Web -->  --> LB
    Mobile  
   
  PI]
    end[Maps A     Maps   I Gateway]
UPI[UP       saging]
  Mesebase Cloud FCM[Firth]
       rebase Aurebase[Fi   Fi    rvices"
 xternal Seh "E subgrap
    
   e)]
    endagStors[(File      Fileache)]
   edis C Redis[(R    