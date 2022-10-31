import React from 'react';
import axios from 'axios';
import { useAppDispatch, useAppSelector, RootState } from '..';
import { showError, showSuccess } from '../../toasts';
import {
  actions,
  AccountFormValue,
} from '../slices/accounts';
import { api } from '../api';

export const save = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { accounts: { form: { value } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.post(`/api/accounts`, value);
    if (success) {
      dispatch(api.util.invalidateTags(['Account']));
      dispatch(actions.closeForm());
      showSuccess("Saved Account!");
    } else {
      dispatch(actions.setErrors(errors))
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const update = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { accounts: { form: { value, accountId } } } : RootState = getState();

    const { data: { success, resource, errors } } = await axios.put(`/api/accounts/${accountId}`, value);
    if (success) {
      dispatch(api.util.invalidateTags(['Account']));
      dispatch(actions.closeForm());
      showSuccess("Updated account!");
    } else {
      dispatch(actions.setErrors(errors))
    }
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const edit = (accountId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    const { data: { account } } = await axios.get(`/api/accounts/${accountId}`);

    if (account) {
      dispatch(actions.resetForm())
      dispatch(actions.setAccountId(accountId))
      dispatch(actions.updateForm(account));
      dispatch(actions.openForm())
    } else {
      console.error('Could not find account....')
    }
  } catch (error) {
    showError(error.toString())
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const destroy = (accountId : number) => async (dispatch, getState) => {
  dispatch(actions.setLoading(true))

  try {
    await axios.delete(`/api/accounts/${accountId}`);

    dispatch(api.util.invalidateTags(['Account']));
    showSuccess("Removed account!")
  } catch (e) {
    showError(e);
  } finally {
    dispatch(actions.setLoading(false))
  }
}

export const newForm = () => (dispatch) => {
  dispatch(actions.resetForm())
  dispatch(actions.openForm())
}

export function useAccountsState() {
  const state = useAppSelector((state) => ({
    form: state.accounts.form
  }));

  const dispatch = useAppDispatch();

  return {
    state,
    actions: {
      updateForm: (value : AccountFormValue) => dispatch(actions.updateForm(value)),
      closeForm: () => dispatch(actions.closeForm()),
      newAccount: () => dispatch(newForm()),
      editAccount: (id) => dispatch(edit(id)),
      destroy: (id) => dispatch(destroy(id)),
      save: () => dispatch(save()),
      update: () => dispatch(update())
    }
  }
}
