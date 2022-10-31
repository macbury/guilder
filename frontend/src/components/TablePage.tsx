import React from 'react';
import CloseIcon from '@rsuite/icons/Close';
import CheckIcon from '@rsuite/icons/Check';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import IconButton from 'rsuite/IconButton';
import PlusIcon from '@rsuite/icons/Plus';
import Grid from 'rsuite/Grid';
import Row from 'rsuite/Row';
import Col from 'rsuite/Col';
import ButtonToolbar from 'rsuite/ButtonToolbar';

export const MarginRow = styled(Row)`
  margin-bottom: 25px;
`;

export const StatusRow = styled(Row)`

`;

export const StatusCol = styled(Col)`
  padding-top: 10px;
  padding-left: 20px;
`;

export const ActionsHeader = styled(Col)`
  display: flex;
  flex-direction: row;
`;

export const Actions = styled(Col)`
  justify-content: end;
  display: flex;
`;

export const ContentRow = styled(Row)`
  flex: 1;
`;

export const FormRow = styled(Row)`
  flex: 1;
  max-width: 1280px;
  margin: 0 auto;
  width: 100%;
`;

export const Gap = styled.div`
  flex: 1;
`;

export const Container = styled(Grid)`
  height: 100%;
  width: 100%;
  flex-direction: column;
  display: flex;
`;

export const FullHeightCol = styled(Col)`
  height: 100%;

  .rs-panel, .rs-panel-body {
    height: 100%;
  }
`;

export const BatchActions = styled.div`
  display: inline-block;
  margin-right: 15px;
`;

export function AddActionButton({ children, ...props }) {
  return <IconButton color="green" appearance="primary" icon={<PlusIcon />} size="lg" {...props}>{children}</IconButton>
}

export function AddActionLink({ to, children, ...props }) {
  return <AddActionButton as={Link} to={to} {...props}>{children}</AddActionButton>
}

export const FormActionButtons = styled(ButtonToolbar)`
  margin-top: 25px;
`;

export function CancelButton({ children, to, ...props }) {
  return <IconButton placement="left" as={Link} to={to} icon={<CloseIcon />} size="lg" {...props}>{children}</IconButton>
}

export function OKButton({ children, ...props }) {
  return <IconButton placement="right" color="green" appearance="primary" icon={<CheckIcon />} size="lg" {...props}>{children}</IconButton>
}
