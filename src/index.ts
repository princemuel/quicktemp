import express from 'express';
import { Db } from 'mongodb';
import { connectToDb, getDb } from './db';

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
  res.json({ message: 'welcome to the api' });
});
