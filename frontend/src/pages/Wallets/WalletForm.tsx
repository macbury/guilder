import React from 'react';
import FlexboxGrid from 'rsuite/FlexboxGrid';
import Panel from 'rsuite/Panel';
import Col from 'rsuite/Col';
import Form from 'rsuite/Form';
import Input from 'rsuite/Input';

import CurrencySelect from '../../components/inputs/CurrencySelect';
import { FormRow, FormActionButtons, CancelButton, OKButton } from '../../components/TablePage';
import { Wallet } from '../../store/api';

export type FormValue = Omit<Wallet, 'id'>;

export interface IWalletFormProps {
  loading: boolean,
  value: FormValue,
  errors: any,
  setValue(value : FormValue);
  submit();
}

const Textarea = React.forwardRef((props, ref) => <Input {...props} as="textarea" ref={ref} />);

export default function WalletForm({ loading, submit, value, setValue, errors } : IWalletFormProps) {
  return (
    <FormRow>
      <Col xs={24} smPush={4} sm={16} mdPush={2} md={18}>
        <Panel shaded bordered bodyFill>
          <Panel>
            <Form
              onSubmit={submit}
              disabled={loading}
              fluid
              formValue={value}
              onChange={setValue}
            >
              <Form.Group controlId="name">
                <Form.ControlLabel>Name:</Form.ControlLabel>
                <Form.Control
                  errorMessage={errors['name']?.join(', ')}
                  name="name" />
              </Form.Group>

              <Form.Group controlId="currency">
                <Form.ControlLabel>Currency:</Form.ControlLabel>
                <Form.Control
                  size="lg"
                  block
                  accepter={CurrencySelect}
                  errorMessage={errors['currency']?.join(', ')}
                  name="currency" />
              </Form.Group>

              <Form.Group controlId="description">
                <Form.ControlLabel>Description</Form.ControlLabel>
                <Form.Control name="description" rows={5} accepter={Textarea} />
              </Form.Group>
            </Form>
          </Panel>
        </Panel>
        <FlexboxGrid justify="end">
          <FormActionButtons>
            <CancelButton to="/wallets" disabled={loading}>Cancel</CancelButton>
            <OKButton loading={loading} onClick={submit}>Save</OKButton>
          </FormActionButtons>
        </FlexboxGrid>
      </Col>
    </FormRow>
  )
}
