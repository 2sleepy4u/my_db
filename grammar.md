## custom type definition

type Book = {
    title: String,
    publish_date: Date,
    n_pages: int,
    author: Author,
    status: BookStatus
    }

type Author = {
    name: String,
    date_of_birth: Date
    }

type BookStatus = Available | NotAvailable

## creation of table

### ex: 1

table Books: { title: String, n_pages: Int }

### ex: 2

table Books: Book


## query

- Query all books 

1. query = Books
2. query: Book = Books
3. query getBooks: Book = Books

- Get all books titles

query: { title: String } = Books

- Query books by title

query getBooksByTitle: Book ( title: String ) = Books[title]

query: Book ( title: String ) = Books[ title = title ]

> Then you just call: getBooksByTitle _"Una notte d'inverno un viaggiatore"_   
> Is also valid: getBooksByTitle ( _"Senilità"_ )  
> And: getBooksByTitle ( title = _"Shibumi"_ )

- Query author by name

query getAuthorByName: Author ( name: String ) = Author[name]

- Query books by author name

query getBooksByAuthor: Book ( name: String ) = 
    getAuthorByName name 

## insert 

insert insertBook ( book: Book ) = Books

## update

update updateBook ( book: Book ) = Books

update updateBook ( title: String, n_pages: Int ) = 
    Books[title, n_pages]

## delete

remove deleteBook ( title: String ) = Books[title]



