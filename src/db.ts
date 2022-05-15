import { Db, MongoClient } from 'mongodb';

// Connection URL
const url = 'mongodb://localhost:27017';

// Database Name
const dbName = 'bookstore';

let dbConnection: Db;

export const connectToDb = (next: (arg?: any) => any) => {
  MongoClient.connect(`${url}/${dbName}`)
    .then((client) => {
      dbConnection = client.db();
      return next();
    })
    .catch((err) => {
      console.log(err);
      return next(err);
    });
};

export const getDb = () => dbConnection;

// // Connection URL
// const client = new MongoClient(url);

// async function main() {
//   // Use connect method to connect to the server
//   await client.connect();
//   console.log('Connected successfully to server');
//   const db = client.db(dbName);
//   const collection = db.collection('documents');

//   // the following code examples can be pasted here...

//   return 'done.';
// }

// main()
//   .then(console.log)
//   .catch(console.error)
//   .finally(() => client.close());
