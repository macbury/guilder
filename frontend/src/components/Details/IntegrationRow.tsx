import React from 'react'
import { Row } from './Row';
import { useLoadIntegrations, useIntegration } from '../../store/hooks/integrations';

export interface IIntegrationRowProps {
  name: string,
  integrationId: number
}

export function IntegrationRow({ name, integrationId, ...props } : IIntegrationRowProps) {
  useLoadIntegrations();
  const integration = useIntegration(integrationId);

  return (
    <Row name={name} {...props}>
      {integration ? integration.name : '-'}
    </Row>
  )
}
