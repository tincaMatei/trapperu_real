pub const BAD_SEPARATORS: &str = "N-ai pus '~' sau ai pus prea multi bombardiere";
pub const BAD_PARANTHESES: &str = "Ai belit parantezele la expresie bombardiere";
pub const BAD_CHARACTERS: &str = "Nush ce plm ai facut dar nu era corect";
pub const BAD_OPERATOR: &str = "Wtf is this";

pub const HELP_DEFAULT: &str = "Cel mai adevarat bot, va arat cum se face smecherie.

Lectii in smecherie, pe capitole (scrie randurile alea complet ca sa vezi capitolul). \
Cea mai importanta este clar alias, cititi-o pe aia mai intai ca aia e importanta.

/help alias
/help joaco
/help adauga
/help taci
/help help
/help gind

Celalalte comenzi de pe acolo care mai apar momentan sunt la harneala, mai aveti rabdare
";

pub const HELP_ALIAS_TAB: &str = "alias";
pub const HELP_ALIAS: &str = "Tin minte o porecla acolo pt grup ca sa ne fie la toti mai usor

/alias
Aici iti zic id-ul grupului si aliasul grupului.

/alias [nume]
Setez porecla grupului in [nume]. Vezi ca asta nu apare nicaieri pe telegram sau ceva, \
doar il tin eu minte sa stiu cum dreq sa va strig. Toate celalalte comenzi se vor folosi \
de aliasul asta
";

pub const HELP_HELP_TAB: &str = "help";
pub const HELP_HELP: &str = "Te ajut in pula mea calmeaza-te";

pub const HELP_TACI_TAB: &str = "taci";
pub const HELP_TACI: &str = "Incearca sa vezi ce face /taci, hai coaie te provoc, nu te tine";

pub const HELP_JOACO_TAB: &str = "joaco";
pub const HELP_JOACO: &str = "[LYRICS]
(Azteca)
Sa moara familia mea
(Ian)
lasama
(Azteca)
Sa moara familia mea
(Ian)
lasama ba lasama
(???)
BA BAAA
(Ian)
Ce-i cu figurile astea pa tine?
(Oscar)
Ba, ba! Joaco!
(Ian)
Leilaaa! Leilalala leila leilaa aaaaaaaaaaaaaaaaaaaaaaaa";

pub const HELP_ADAUGA_TAB: &str = "adauga";
pub const HELP_ADAUGA: &str = "Fii atent coaie te arat cum se face smecherie cu adaugatul.

Ai urmatoarele variante:
/adauga [Expresie]~[Mesaj]
/adauga [Alias]~[Expresie]~[Mesaj]

[Mesaj] este un mesaj oarecare cu care raspund daca expresia [Expresie] este adevarata.

Unde la [Expresie] ai ceva de genul \"a&b|(c|d&e)\", unde a, b, c, d si e sunt chestii cu \
litere si cifre, doar astea, sa nu aiba altele dintre care si spatii (daca bagi spatii \
vezi ca le ignor direct si daca ai o expresie \"a b\", atunci se triggereste daca zici ab). \
Daca expresia este corecta dpdv gramatical, atunci cand cineva trimite un mesaj, eu o sa \
inlocuiesc fiecare cuvant din expresia aia cu adevarat daca apare sau fals daca nu apare \
si daca la sfarsit expresia este adevarata, atunci o sa zic mesajul de mai sus.

De exemplu, daca dau comanda \"/adauga a&(b|c)~test\", o sa raspund cu \"test\" daca cineva \
zice cuvantul a si unul din cuvintele b si c absolut oriunde in propozitie.

[Alias] este porecla grupului, daca vrei sa bagi o comanda si sa nu vada ceilalti, intri \
la mine in DM si folosesti varianta a doua de mai sus cu aliasul ala (da vezi ca trebuie \
sa setezi aliasul ala cand ma bagi pe grup). Daca ai uitat aliasul sau nu stii cum, \
doar dai '/alias' si gata te-ai scos.";

pub const WRONG_ALIAS: &str = "Ceai facut bombardiere, ai gresit aliasul?";

pub const HELP_GIND_TAB: &str = "gind";
pub const HELP_GIND: &str = "Aici ai doua comenzi importante, una din ele avand si: \
ea doua variante.

/gindeste [Mesaj]
/gindeste [Alias]~[Mesaj]
/gind

Daca dai prima varianta, o sa ascult un gind pe care sa-l tin minte in viitor in grupul \
de pe care ai dat comanda.
Daca dai a doua varianta, o sa fac acelasi lucru ca prima, numai ca pe grupul cu aliasul \
dat, gen ca si la /adauga.
Daca dai /gind, iti dau un gind frumos pe care l-am tinut minte si il zic pe grup, dar \
vezi ca nu o sa zic acelasi gind de doua ori ca nu sunt fraier";

