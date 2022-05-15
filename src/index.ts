import express from 'express';

const app = express();
const PORT = process.env.PORT || 3500;

app.get('/books', (req, res) => {
  res.json({ message: 'welcome to the api' });
});

app.listen(PORT, () => {
  console.log(`app listening on port ${PORT}`);
});
