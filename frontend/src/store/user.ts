import { atom } from 'jotai';

type UserInfo = {
  user_id?: number
}

export const userAtom = atom<UserInfo>({});

