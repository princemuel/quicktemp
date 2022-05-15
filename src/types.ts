import { WithId } from 'mongodb';

export type IBooks = WithId<IBook>[];

export interface IBook {
  title: string;
  author: string;
  rating: number;
  pages: number;
  genres: string[];
  reviews: Review[];
}

interface Review {
  name: string;
  body: string;
}
