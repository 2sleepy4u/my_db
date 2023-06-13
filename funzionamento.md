type Book = {
    title: String,
    publish_date: Date,
    n_pages: int,
    author: Author
    }

type Author = {
    name: String,
    date_of_birth: Date
    }

Dentro al tipo Book c'è il tipo Author ma non è come se fosse un oggetto contenuto in un altro oggetto.  
I tipi custom, quando all'interno di un altro tipo custom, sono semplicemente il riferimento / indirizzo / puntatore che punta al tipo di riferimento.  

Quindi il campo **author** all'interno di **Book** è l'indirizzo ad un tipo Author.




