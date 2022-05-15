import express from 'express';
import { Db } from 'mongodb';
import { connectToDb, getDb } from './db';
import { IBook, IBooks } from './types';

const app = express();
const PORT = process.env.PORT || 3500;

// db connection
let db: Db;

connectToDb((err) => {
  if (!err) {
    app.listen(PORT, () => {
      console.log(`app listening on port ${PORT}`);
    });
    db = getDb();
  }
});

app.get('/books', (req, res) => {
  let books: IBooks = [];

  db.collection<IBook>('books')
    .find()
    .sort({ author: 1 })
    // @ts-expect-error
    .forEach((book) => books.push(book))
    .then(() => {
      res.status(200).json(books);
    })
    .catch(() => {
      res.status(500).json({ error: 'could not fetch the documents' });
    });
});
