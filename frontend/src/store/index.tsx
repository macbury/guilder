import React from 'react';
import { Provider, TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';
import { configureStore } from '@reduxjs/toolkit';
import { setupListeners } from '@reduxjs/toolkit/query/react'
import { api } from './api'
import session from './slices/session';
import categories from './slices/categories';
import accounts from './slices/accounts';
import integrations from './slices/integrations';
import bonds from './slices/bonds';
import assets from './slices/assets';
import comments from './slices/comments';

export const store = configureStore({
  devTools: true,
  reducer: {
    [api.reducerPath]: api.reducer,
    session,
    categories,
    accounts,
    integrations,
    bonds,
    assets,
    comments
  },

  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware().concat(api.middleware),
})

setupListeners(store.dispatch)

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch

export const useAppDispatch = () => useDispatch<AppDispatch>()
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector

export default function StoreProvider({ children }) {
  return (
    <Provider store={store}>
      {children}
    </Provider>
  )
}
