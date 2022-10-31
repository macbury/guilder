import React, { useEffect, useMemo } from 'react';
import axios from 'axios';
import { bindActionCreators } from '@reduxjs/toolkit';
import { useAppDispatch, useAppSelector, RootState } from '..';
import useAbortCallback from '../../hooks/useAbortCallback';
import {
  commentsAdapter,
  actions,
  Comment,
} from '../slices/comments';
import { fetchAll, destroy } from '../api/comments';

export const {
  selectAll,
  selectById,
} = commentsAdapter.getSelectors<RootState>((state) => state.comments);

export const create = () => async (dispatch, getState) => {
  const state : RootState = getState();
  const { comments } = state;

  dispatch(actions.setLoading(true));

  try {
    const { data: { success, resource: comment, errors } } = await axios.post(`/api/comments/`, {
      body: comments.body,
      modelId: comments.modelId,
      modelType: comments.modelType
    });

    if (success) {
      dispatch(actions.addComment(comment))
      dispatch(actions.setBody(''));
      dispatch(actions.setErrors({}));
    } else {
      dispatch(actions.setErrors(errors));
    }
  } catch (error) {
    if (error.name !== "AbortError") {
      console.error("Could not fetch comments", error);
    }
  } finally {
    dispatch(actions.setLoading(false));
  }
}

export function useCommentsState() {
  const dispatch = useAppDispatch();
  const state = useAppSelector((state) => ({
    loading: state.comments.loading,
    errors: state.comments.errors,
    body: state.comments.body,
    comments: selectAll(state),
  }));

  const boundActions = useMemo(() => bindActionCreators(({
    ...actions,
    create,
    fetchAll,
    destroy
  }), dispatch), [dispatch])

  return {
    state,
    actions: boundActions
  }
}

export function useLoadComments(modelId : string, modelType : string) {
  const {
    state,
    actions
  } = useCommentsState();

  const refresh = useAbortCallback((signal, modelId, modelType) => {
    const params = {
      modelId, modelType
    }
    actions.setModel(params);
    actions.fetchAll({ signal, params });
  }, []);

  useEffect(() => {
    return refresh(modelId, modelType)
  }, [modelId, modelType])

  return { state, actions }
}
