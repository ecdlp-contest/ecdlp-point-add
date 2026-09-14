// Construction-value and allocator support for the source-to-Rust backend.
// This authored compiler runtime contains no arithmetic program or gate table.
pub use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::collections::{BTreeMap,BTreeSet};
use quantum_ecc::circuit::*;

#[derive(Clone)]
pub enum V {
    None,I(i128),S(String),L(Rc<RefCell<Vec<V>>>),D(Rc<RefCell<BTreeMap<String,V>>>),
    Fun(usize,Weak<Environment>),Native(String),Bound(Box<V>,String),Compiler,
}
pub type Args=Vec<(String,V)>;
pub struct Environment { values:RefCell<BTreeMap<String,V>>, parent:Option<E> }
#[derive(Clone)] pub struct E(pub Rc<Environment>);
impl E {
    pub fn new()->Self{Self(Rc::new(Environment{values:RefCell::new(BTreeMap::new()),parent:None}))}
    pub fn child(&self)->Self{Self(Rc::new(Environment{values:RefCell::new(BTreeMap::new()),parent:Some(self.clone())}))}
    pub fn set(&self,k:&str,v:V){self.0.values.borrow_mut().insert(k.to_string(),v);}
    pub fn get(&self,k:&str)->V{
        if let Some(v)=self.0.values.borrow().get(k){return v.clone()}
        if let Some(p)=&self.0.parent{return p.get(k)}
        match k {"len"|"range"|"list"|"set"|"reversed"|"zip"|"enumerate"|"int"|"str"|"max"|"min"|"sum"|"dict"|"all"|"any"|"abs"|"bool"=>V::Native(k.into()),_=>panic!("unbound source name {k}")}
    }
}
pub fn bind(e:&E,args:Args,names:&[&str],defaults:Vec<V>){
    let mut used=vec![false;names.len()];let mut pos=0;
    for (k,v) in args {
        let i=if k.is_empty(){let i=pos;pos+=1;i}else{names.iter().position(|n|*n==k).expect("unknown keyword")};
        assert!(i<names.len()&&!used[i],"invalid arguments");used[i]=true;e.set(names[i],v);
    }
    for i in 0..names.len(){if !used[i]{assert!(i>=names.len()-defaults.len(),"missing {}",names[i]);e.set(names[i],defaults[i-(names.len()-defaults.len())].clone());}}
}
pub fn vs(s:&str)->V{V::S(s.into())}
pub fn vl(v:Vec<V>)->V{V::L(Rc::new(RefCell::new(v)))}
pub fn vd(v:Vec<(V,V)>)->V{V::D(Rc::new(RefCell::new(v.into_iter().map(|(k,v)|(string(k),v)).collect())))}
pub fn vb(b:bool)->V{V::I(b as i128)}
pub fn integer(v:V)->i128{match v{V::I(i)=>i,_=>panic!("expected integer, got {}",string(v))}}
pub fn string(v:V)->String{match v{V::None=>"None".into(),V::I(i)=>i.to_string(),V::S(s)=>s,V::L(l)=>format!("[{}]",l.borrow().iter().map(|v|string(v.clone())).collect::<Vec<_>>().join(",")),V::D(_)=>"dict".into(),_=>"object".into()}}
pub fn truth(v:V)->bool{match v{V::None=>false,V::I(i)=>i!=0,V::S(s)=>!s.is_empty(),V::L(l)=>!l.borrow().is_empty(),V::D(d)=>!d.borrow().is_empty(),_=>true}}
pub fn iter(v:V)->Vec<V>{match v{V::L(l)=>l.borrow().clone(),V::D(d)=>d.borrow().keys().map(|k|vs(k)).collect(),_=>panic!("not iterable: {}",string(v))}}
fn equal(a:V,b:V)->bool{match(a,b){(V::None,V::None)=>true,(V::I(a),V::I(b))=>a==b,(V::S(a),V::S(b))=>a==b,(V::L(a),V::L(b))=>{let a=a.borrow();let b=b.borrow();a.len()==b.len()&&a.iter().zip(b.iter()).all(|(x,y)|equal(x.clone(),y.clone()))},(V::D(a),V::D(b))=>{let a=a.borrow();let b=b.borrow();a.len()==b.len()&&a.iter().all(|(k,x)|b.get(k).is_some_and(|y|equal(x.clone(),y.clone())))},_=>false}}
fn contains(container:V,item:V)->bool{match container{V::D(d)=>d.borrow().contains_key(&string(item)),V::L(l)=>l.borrow().iter().any(|x|equal(x.clone(),item.clone())),V::S(s)=>s.contains(&string(item)),_=>panic!("invalid contains")}}
pub fn compare(op:&str,a:V,b:V)->bool{match op{
    "eq"|"is"=>equal(a,b),"ne"|"isnot"=>!equal(a,b),"in"=>contains(b,a),"notin"=>!contains(b,a),
    "lt"=>integer(a)<integer(b),"le"=>integer(a)<=integer(b),"gt"=>integer(a)>integer(b),"ge"=>integer(a)>=integer(b),_=>panic!("compare {op}")}}
pub fn binary(op:&str,a:V,b:V)->V{
    if op=="add"{match(&a,&b){(V::L(_),V::L(_))=>{let mut v=iter(a);v.extend(iter(b));return vl(v)},(V::S(a),V::S(b))=>return vs(&(a.clone()+b)),_=>{}}}
    if op=="mul"{if matches!(a,V::L(_)){let n=integer(b);let a=iter(a);return vl((0..n).flat_map(|_|a.clone()).collect())}}
    let a=integer(a);let b=integer(b);
    V::I(match op{"add"=>a.checked_add(b).expect("integer overflow"),"sub"=>a.checked_sub(b).expect("integer overflow"),"mul"=>a.checked_mul(b).expect("integer overflow"),"div"=>a.div_euclid(b),"mod"=>a.rem_euclid(b),"shl"=>{assert!((0..127).contains(&b));a.checked_shl(b as u32).unwrap()},"shr"=>{assert!(b>=0);if b>=128 {if a<0{-1}else{0}} else {a>>b}},"and"=>a&b,"or"=>a|b,"xor"=>a^b,"pow"=>a.checked_pow(b as u32).unwrap(),_=>panic!("binary {op}")})
}
fn index(i:i128,n:usize)->usize{let i=if i<0{n as i128+i}else{i};assert!(i>=0&&(i as usize)<n,"index {i} out of {n}");i as usize}
pub fn get(v:V,k:V)->V{match v{V::L(l)=>{let l=l.borrow();l[index(integer(k),l.len())].clone()},V::D(d)=>d.borrow().get(&string(k.clone())).unwrap_or_else(||panic!("missing key {}",string(k))).clone(),_=>panic!("not indexable")}}
pub fn put(v:V,k:V,x:V){match v{V::L(l)=>{let mut l=l.borrow_mut();let i=index(integer(k),l.len());l[i]=x},V::D(d)=>{d.borrow_mut().insert(string(k),x);},_=>panic!("not assignable")}}
pub fn slice(v:V,lo:V,hi:V,step:V)->V{
    assert!(matches!(step,V::None)||integer(step)==1);let v=iter(v);let n=v.len() as i128;
    let bound=|v:V,default:i128|{if matches!(v,V::None){default}else{let i=integer(v);(if i<0{n+i}else{i}).clamp(0,n)}};
    let a=bound(lo,0) as usize;let b=bound(hi,n) as usize;vl(if a>b{vec![]}else{v[a..b].to_vec()})
}
pub fn attribute(c:&mut B,v:V,k:&str)->V{
    if matches!(v,V::Compiler){match k{
        "guards"=>return c.guards.clone(),"live"=>return vl(c.live.iter().map(|&q|V::I(q as i128)).collect()),"lent"=>return vl(c.lent.iter().map(|&q|V::I(q as i128)).collect()),
        "phase_name"|"phase_limit"|"fold_low"|"layout"=>return c.fields.get(k).unwrap().clone(),
        _=>return V::Bound(Box::new(v),k.into())
    }}
    V::Bound(Box::new(v),k.into())
}
pub fn set_attribute(c:&mut B,v:V,k:&str,x:V){assert!(matches!(v,V::Compiler));c.fields.insert(k.into(),x);}

pub fn invoke(c:&mut B,f:V,args:Args)->V{match f{
    V::Fun(id,env)=>super::arithmetic::dispatch(c,id,E(env.upgrade().expect("expired source closure")),args),
    V::Native(name)=>builtin(&name,args),V::Bound(v,name)=>method(c,*v,&name,args),_=>panic!("not callable")
}}
fn builtin(name:&str,args:Args)->V{
    if name=="dict"{return vd(args.into_iter().map(|(k,v)|(vs(&k),v)).collect())}
    let a:Vec<V>=args.into_iter().map(|(_,v)|v).collect();
    match name{
        "len"=>V::I(match &a[0]{V::S(s)=>s.len(),V::L(l)=>l.borrow().len(),V::D(d)=>d.borrow().len(),_=>panic!("len") }as i128),
        "int"=>V::I(match &a[0]{V::I(i)=>*i,V::None=>panic!("int None"),V::S(s)=>s.parse().unwrap(),_=>panic!("int")}),
        "bool"=>vb(truth(a[0].clone())),"str"=>vs(&string(a[0].clone())),"abs"=>V::I(integer(a[0].clone()).abs()),
        "range"=>{let (start,stop,step)=match a.len(){1=>(0,integer(a[0].clone()),1),2=>(integer(a[0].clone()),integer(a[1].clone()),1),3=>(integer(a[0].clone()),integer(a[1].clone()),integer(a[2].clone())),_=>panic!("range")};assert!(step!=0);let mut out=vec![];let mut i=start;while if step>0{i<stop}else{i>stop}{out.push(V::I(i));i+=step;}vl(out)},
        "list"=>vl(iter(a[0].clone())),"set"=>{let mut out=vec![];for v in iter(a[0].clone()){if !out.iter().any(|x: &V|equal(x.clone(),v.clone())){out.push(v)}}vl(out)},
        "reversed"=>{let mut v=iter(a[0].clone());v.reverse();vl(v)},
        "enumerate"=>vl(iter(a[0].clone()).into_iter().enumerate().map(|(i,v)|vl(vec![V::I(i as i128),v])).collect()),
        "zip"=>{let rows:Vec<Vec<V>>=a.into_iter().map(iter).collect();let n=rows.iter().map(|v|v.len()).min().unwrap();vl((0..n).map(|i|vl(rows.iter().map(|v|v[i].clone()).collect())).collect())},
        "min"|"max"=>{let vals=if a.len()==1{iter(a[0].clone())}else{a};let mut nums=vals.into_iter().map(integer);let first=nums.next().unwrap();V::I(nums.fold(first,|a,b|if name=="min"{a.min(b)}else{a.max(b)}))},
        "sum"=>V::I(iter(a[0].clone()).into_iter().map(integer).sum()),
        "all"=>vb(iter(a[0].clone()).into_iter().all(truth)),"any"=>vb(iter(a[0].clone()).into_iter().any(truth)),
        _=>panic!("unknown builtin {name}")
    }
}
fn method(c:&mut B,v:V,name:&str,args:Args)->V{
    if matches!(v,V::Compiler){return c.call(name,args)}
    let a:Vec<V>=args.into_iter().map(|(_,v)|v).collect();
    match (v,name){
        (V::L(l),"append")=>{l.borrow_mut().push(a[0].clone());V::None},
        (V::L(l),"pop")=>{let mut l=l.borrow_mut();let i=if a.is_empty(){l.len()-1}else{index(integer(a[0].clone()),l.len())};l.remove(i)},
        (V::L(l),"index")=>{let start=if a.len()>1{integer(a[1].clone())as usize}else{0};V::I(l.borrow().iter().enumerate().skip(start).find(|(_,v)|equal((*v).clone(),a[0].clone())).unwrap().0 as i128)},
        (V::L(l),"copy")=>vl(l.borrow().clone()),
        (V::D(d),"pop")=>d.borrow_mut().remove(&string(a[0].clone())).expect("missing pop"),
        (V::S(s),"endswith")=>vb(s.ends_with(&string(a[0].clone()))),
        (V::I(i),"bit_length")=>V::I(128-i.unsigned_abs().leading_zeros() as i128),
        _=>panic!("unsupported method {name}")
    }
}

pub struct B {
    pub ops:Vec<Op>,pub live:BTreeSet<u64>,pub lent:BTreeSet<u64>,free:BTreeSet<u64>,next:u64,bits:u64,
    pub guards:V,pub fields:BTreeMap<String,V>,pub source:E, scratch:Vec<u64>,
    events:u64,peak:usize,
}
impl B {
    pub fn new()->Self{
        Self{ops:Vec::with_capacity(21_213_277),live:(0..512).collect(),lent:BTreeSet::new(),free:BTreeSet::new(),next:512,bits:512,guards:vl(vec![]),
            fields:BTreeMap::from([("phase_name".into(),vs("abi")),("phase_limit".into(),V::I(256)),("fold_low".into(),V::None),("layout".into(),vb(false))]),
            source:E::new(),scratch:vec![],events:0,peak:512}
    }
    fn owner(&mut self,_action:u8,_q:u64){self.events+=1;}
    fn alloc(&mut self)->u64{
        let q=if let Some(q)=self.free.pop_first(){q}else{let q=self.next;self.next+=1;q};
        assert!(self.live.insert(q));self.peak=self.peak.max(self.live.len());self.owner(0,q);q
    }
    fn release(&mut self,q:u64){assert!(self.live.contains(&q),"release {q}");self.owner(1,q);self.live.remove(&q);assert!(self.free.insert(q));}
    fn take(&mut self,q:u64,action:u8){assert!(!self.live.contains(&q)&&self.free.remove(&q));self.live.insert(q);self.peak=self.peak.max(self.live.len());self.owner(action,q);}
    fn record(&mut self,kind:OperationType,qs:&[u64],bit:Option<u64>,reg:Option<u64>,guards:Vec<u64>){
        assert!(guards.len()<=2);assert!(qs.iter().all(|q|self.live.contains(q)));assert_eq!(qs.iter().copied().collect::<BTreeSet<_>>().len(),qs.len());
        for &g in &guards{let mut o=Op::empty();o.kind=OperationType::PushCondition;o.c_condition=BitId(g);self.ops.push(o);}
        let mut o=Op::empty();o.kind=kind;
        if let Some(&q)=qs.last(){o.q_target=QubitId(q)}
        if qs.len()>=2{o.q_control1=QubitId(qs[qs.len()-2])}
        if qs.len()==3{o.q_control2=QubitId(qs[0])}
        if let Some(b)=bit{o.c_target=BitId(b)}if let Some(r)=reg{o.r_target=RegisterId(r)}self.ops.push(o);
        for _ in guards{let mut o=Op::empty();o.kind=OperationType::PopCondition;self.ops.push(o);}
    }
    fn guard_ids(&self)->Vec<u64>{iter(self.guards.clone()).into_iter().map(|v|integer(v)as u64).collect()}
    fn measure(&mut self,q:u64)->u64{let m=self.bits;self.bits+=1;self.record(OperationType::Hmr,&[q],Some(m),None,self.guard_ids());m}
    fn call(&mut self,name:&str,args:Args)->V{
        if matches!(name,"recipe"|"chunk_add"|"compare_phase") {let f=self.source.get(name);return invoke(self,f,args)}
        let a:Vec<V>=args.into_iter().map(|(_,v)|v).collect();
        match name{
            "alloc"=>V::I(self.alloc()as i128),"release"=>{self.release(integer(a[0].clone())as u64);V::None},
            "loan"=>{let q=integer(a[0].clone())as u64;assert!(self.live.contains(&q)&&!self.lent.contains(&q));self.owner(2,q);self.live.remove(&q);self.lent.insert(q);assert!(self.free.insert(q));V::None},
            "take_at"=>{self.take(integer(a[0].clone())as u64,0);V::None},
            "reclaim"=>{let q=integer(a[0].clone())as u64;assert!(self.lent.remove(&q));self.take(q,3);V::None},
            "measure"=>V::I(self.measure(integer(a[0].clone())as u64)as i128),
            "emit"=>{let name=string(a[0].clone());let kind=match name.as_str(){"x"=>OperationType::X,"z"=>OperationType::Z,"cx"=>OperationType::CX,"cz"=>OperationType::CZ,"ccx"=>OperationType::CCX,_=>panic!("gate {name}")};let qs:Vec<u64>=a[1..].iter().map(|x|integer(x.clone())as u64).collect();self.record(kind,&qs,None,None,self.guard_ids());V::None},
            "bit"=>{let b=self.bits;self.bits+=1;self.scratch.push(b);V::I(b as i128)},
            "cop"=>{let name=string(a[0].clone());let kind=match name.as_str(){"bit_zero"=>OperationType::BitStore0,"bit_one"=>OperationType::BitStore1,"bit_invert"=>OperationType::BitInvert,_=>panic!("classical gate {name}")};let b=integer(a[1].clone())as u64;let guards=if a.len()>2{iter(a[2].clone()).into_iter().map(|v|integer(v)as u64).collect()}else{vec![]};assert!(b>=512&&b<self.bits&&self.guard_ids().is_empty());self.record(kind,&[],Some(b),None,guards);V::None},
            "declare"=>{for r in 0..4{self.record(OperationType::Register,&[],None,Some(r),vec![]);for i in 0..256{let id=(r%2)*256+i;if r<2{self.record(OperationType::AppendToRegister,&[id],None,Some(r),vec![])}else{self.record(OperationType::AppendToRegister,&[],Some(id),Some(r),vec![])}}}V::None},
            "clear_classical"=>{for b in self.scratch.clone(){self.record(OperationType::BitStore0,&[],Some(b),None,vec![])}V::None},
            "checkpoint"=>V::None,
            _=>panic!("unknown construction method {name}")
        }
    }
    pub fn finish(mut self)->Vec<Op>{
        assert_eq!(self.live,(0..512).collect());assert!(self.lent.is_empty()&&!truth(self.guards.clone()));
        self.ops
    }
}
