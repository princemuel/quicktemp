import express from 'express';
import { Collection, Db, ObjectId } from 'mongodb';
import { connectToDb, getDb } from './db';
import { IBook, IBooks } from './types';

const app = express();
const PORT = process.env.PORT || 3500;

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// db connection
let db: Db;
let booksCollection: Collection<IBook>;
const booksApi = '/api/books';

connectToDb((err) => {
  if (!err) {
    app.listen(PORT, () => {
      console.log(`app listening on port ${PORT}`);
    });
    db = getDb();
    booksCollection = db.collection('books');
  }
});

app.get(`${booksApi}`, (req, res) => {
  const page = req.query?.page || 0;
  const booksPerPage = req.query?.limit || 3;

  let books: IBooks = [];

  booksCollection
    .find()
    .sort({ author: 1 })
    .skip(+page * +booksPerPage)
    .limit(+booksPerPage)
    // @ts-expect-error
    .forEach((book) => books.push(book))
    .then(() => {
      res.status(200).json(books);
    })
    .catch((err) => {
      res.status(500).json({ error: 'could not fetch this books' });
    });
});

app.get(`${booksApi}/:id`, (req, res) => {
  const { id } = req.params;

  // NOTE:  find a way to check if the id is a valid BSON ObjectId
  // but is not present in the database
  if (ObjectId.isValid(id)) {
    booksCollection
      .findOne({ _id: new ObjectId(id) })
      .then((book) => {
        res.status(200).json(book);
      })
      .catch((err) => {
        res.status(500).json({ error: 'could not fetch this book' });
      });
  } else {
    res.status(500).json({ error: "this book's id is not a valid id" });
  }
});

app.post(`${booksApi}`, (req, res) => {
  const book = req.body as IBook;

  booksCollection
    .insertOne(book)
    .then((response) => {
      res.status(201).json(response);
    })
    .catch((err) => {
      res.status(500).json({ error: 'could not create a new book' });
    });
});

app.patch(`${booksApi}/:id`, (req, res) => {
  const { id } = req.params;
  const updates = req.body as Partial<IBook>;

  // NOTE:  find a way to check if the id is a valid BSON ObjectId
  // but is not present in the database
  if (ObjectId.isValid(id)) {
    booksCollection
      // .updateOne({ _id: new ObjectId(id) }, {$set:{ ...updates}})
      .updateOne({ _id: new ObjectId(id) }, { $set: updates })
      .then((response) => {
        res.status(200).json(response);
      })
      .catch((err) => {
        res.status(500).json({ error: 'could not update this book' });
      });
  } else {
    res.status(500).json({ error: "this book's id is not a valid id" });
  }
});

app.delete(`${booksApi}/:id`, (req, res) => {
  const { id } = req.params;

  // NOTE:  find a way to check if the id is a valid BSON ObjectId
  // but is not present in the database
  if (ObjectId.isValid(id)) {
    booksCollection
      .deleteOne({ _id: new ObjectId(id) })
      .then((response) => {
        res.status(200).json(response);
      })
      .catch((err) => {
        res.status(500).json({ error: 'could not delete this book' });
      });
  } else {
    res.status(500).json({ error: "this book's id is not a valid id" });
  }
});
