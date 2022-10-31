import React, { useEffect, useMemo } from 'react';
import sumBy from 'lodash/sumBy';
import { createSelector, bindActionCreators } from '@reduxjs/toolkit';
import { useAppDispatch, useAppSelector, RootState } from '..';
import useAbortCallback from '../../hooks/useAbortCallback';
import {
  bondsAdapter,
  actions,
  Bond,
} from '../slices/bonds';
import * as reduxApi from '../api';
import * as api from '../api/bonds';
import { BondFilterOptions } from '../api/bonds';
import axios from 'axios';
import { showError, showSuccess } from '../../toasts';

export const {
  selectAll,
  selectById
} = bondsAdapter.getSelectors<RootState>((state) => state.bonds);

export const selectFormValue = createSelector((state : RootState) => state.bonds.form, (form) => form.value)

const selectSummary = createSelector((state : RootState) => selectAll(state), (bonds) => {
  let shares = sumBy(bonds, ({ performance }) => performance.shares);
  let startPrice = sumBy(bonds, ({ performance }) => performance.startPrice);
  let currentPrice = sumBy(bonds, ({ performance }) => performance.price);
  let dayPriceChange = sumBy(bonds, ({ performance }) => performance.dayPriceChange);
  let buyoutPrice = sumBy(bonds, ({ performance }) => performance.buyoutPrice);
  let avgRate = sumBy(bonds, ({ performance }) => performance.currentRate) / bonds.length;
  return {
    shares, startPrice, currentPrice, dayPriceChange, avgRate, buyoutPrice
  }
})

export const selectedEntitiesIds = createSelector((state : RootState) => state.bonds.selectedIds, (selectedIds) => {
  return Object.keys(selectedIds).map((id) => {
    if (selectedIds[id]) {
      return parseInt(id)
    } else {
      return null
    }
  }).filter((id) => id != null)
})

export const destroyAll = () => async (dispatch, getState) => {
  if (!confirm("Do you want to destroy selected bonds?")) {
    return
  }

  dispatch(actions.setLoading(true));
  const selectedIds = selectedEntitiesIds(getState());

  for (let index = 0; index < selectedIds.length; index++) {
    const bondId = selectedIds[index];

    try {
      await axios.delete(`/api/bonds/${bondId}`);

      dispatch(actions.removeBond(bondId));
    } catch (error) {
      showError(error.toString())
    }
  }

  dispatch(actions.clearSelection());
  dispatch(actions.setLoading(false));
}

export const massUpdate = () => async (dispatch, getState) => {
  dispatch(actions.setLoading(true));
  dispatch(actions.clearErrors())

  const state = getState();
  const selectedIds = selectedEntitiesIds(state);
  const form = selectFormValue(state);

  for (let index = 0; index < selectedIds.length; index++) {
    const id = selectedIds[index];

    try {
      const { data: { success, resource, errors } } = await axios.put(`/api/bonds/${id}`, form);
      if (success) {
        dispatch(actions.deselect(id));
        dispatch(actions.updateBond(resource));
        showSuccess(`Updated bond: ${resource.emission}`);
      } else {
        dispatch(actions.setErrors(errors))
        return
      }
    } catch (error) {
      showError(error.toString())
      break;
    }
  }

  dispatch(reduxApi.api.util.invalidateTags(['Account', 'Wallet']));
  dispatch(actions.resetForm())
  dispatch(actions.hideForm())
  dispatch(actions.setLoading(false));
}

export const showForm = () => (dispatch) => {
  dispatch(actions.clearErrors())
  dispatch(actions.resetForm())
  dispatch(actions.showForm())
}

export function useBondsState() {
  const dispatch = useAppDispatch();
  const state = useAppSelector((state) => ({
    loading: state.bonds.loading,
    form: state.bonds.form,
    bonds: selectAll(state),
    summary: selectSummary(state),
    selectedIds: state.bonds.selectedIds,
    selectedCount: selectedEntitiesIds(state).length
  }));

  const boundActions = useMemo(() => bindActionCreators(({
    ...actions,
    showForm,
    massUpdate,
    destroyAll,
  }), dispatch), [dispatch])

  return {
    state,
    actions: boundActions
  }
}

export function useSortableBondsState(filter : BondFilterOptions) {
  const dispatch = useAppDispatch();
  const state = useBondsState();

  const refresh = useAbortCallback((signal, params : BondFilterOptions) => {
    dispatch(api.fetchAll({ signal, params }))
  }, []);

  useEffect(() => {
    return refresh(filter);
  }, [filter])

  return state
}


export function useBondState(bondId: number) {
  const dispatch = useAppDispatch();
  const state = useAppSelector((state) => ({
    loading: state.bonds.loading,
    bond: selectById(state, state.bonds.currentBondId || -1)
  }));

  const boundActions = useMemo(() => bindActionCreators(({
    setCurrentBondId: actions.setCurrentBondId,
    fetch: api.fetch
  }), dispatch), [dispatch])

  const refresh = useAbortCallback((signal, bondId: number) => {
    boundActions.setCurrentBondId(bondId);
    boundActions.fetch({ params: { bondId }, signal });
  }, []);

  useEffect(() => {
    return refresh(bondId)
  }, [bondId])

  return {
    state,
    actions: {}
  }
}
