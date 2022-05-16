import express from 'express';
import { Collection, Db, ObjectId } from 'mongodb';
import { connectToDb, getDb } from './db';
import { IBook, IBooks } from './types';

const app = express();
const PORT = process.env.PORT || 3500;

// db connection
let db: Db;
let booksCollection: Collection<IBook>;

connectToDb((err) => {
  if (!err) {
    app.listen(PORT, () => {
      console.log(`app listening on port ${PORT}`);
    });
    db = getDb();
    booksCollection = db.collection('books');
  }
});

app.get('/books', (req, res) => {
  let books: IBooks = [];

  booksCollection
    .find()
    .sort({ author: 1 })
    // @ts-expect-error
    .forEach((book) => books.push(book))
    .then(() => {
      res.status(200).json(books);
    })
    .catch((err) => {
      res.status(500).json({ error: 'could not fetch the documents' });
    });
});

app.get('/books/:id', (req, res) => {
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
        res.status(500).json({ error: 'could not fetch the document' });
      });
  } else {
    res.status(500).json({ error: 'the document id is not a valid id' });
  }
});
