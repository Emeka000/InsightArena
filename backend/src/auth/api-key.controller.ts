import {
  Body,
  Controller,
  Delete,
  Get,
  HttpCode,
  HttpStatus,
  Param,
  ParseUUIDPipe,
  Post,
} from '@nestjs/common';
import {
  ApiBearerAuth,
  ApiOperation,
  ApiResponse,
  ApiTags,
} from '@nestjs/swagger';
import { ApiKeyService } from './api-key.service';
import { CreateApiKeyDto } from './dto/create-api-key.dto';
import {
  ApiKeyCreatedResponseDto,
  ApiKeyListItemDto,
} from './dto/api-key-response.dto';
import { CurrentUser } from '../common/decorators/current-user.decorator';
import { User } from '../users/entities/user.entity';

@ApiTags('API Keys')
@ApiBearerAuth()
@Controller('auth/api-keys')
export class ApiKeyController {
  constructor(private readonly apiKeyService: ApiKeyService) {}

  /**
   * Create a new API key.
   * The raw key is returned exactly once — store it securely, it cannot be retrieved again.
   */
  @Post()
  @HttpCode(HttpStatus.CREATED)
  @ApiOperation({
    summary: 'Create an API key',
    description:
      'Issues a new scoped API key. The raw key is shown once and cannot be retrieved again.',
  })
  @ApiResponse({
    status: 201,
    description: 'Key created — raw key shown once',
    type: ApiKeyCreatedResponseDto,
  })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  create(
    @CurrentUser() user: User,
    @Body() dto: CreateApiKeyDto,
  ): Promise<ApiKeyCreatedResponseDto> {
    return this.apiKeyService.create(user.id, dto);
  }

  /**
   * List all API keys belonging to the authenticated user.
   * Never includes hashes or raw keys.
   */
  @Get()
  @HttpCode(HttpStatus.OK)
  @ApiOperation({ summary: "List the caller's API keys" })
  @ApiResponse({
    status: 200,
    description: 'Array of API key summaries (no secrets)',
    type: [ApiKeyListItemDto],
  })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  list(@CurrentUser() user: User): Promise<ApiKeyListItemDto[]> {
    return this.apiKeyService.listForUser(user.id);
  }

  /**
   * Revoke an API key by ID.
   * Only the key's owner may revoke it.
   */
  @Delete(':id')
  @HttpCode(HttpStatus.OK)
  @ApiOperation({ summary: 'Revoke an API key' })
  @ApiResponse({
    status: 200,
    description: 'Key successfully revoked',
    type: ApiKeyListItemDto,
  })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  @ApiResponse({ status: 403, description: 'Key already revoked' })
  @ApiResponse({ status: 404, description: 'Key not found' })
  revoke(
    @CurrentUser() user: User,
    @Param('id', ParseUUIDPipe) id: string,
  ): Promise<ApiKeyListItemDto> {
    return this.apiKeyService.revoke(id, user.id);
  }
}
