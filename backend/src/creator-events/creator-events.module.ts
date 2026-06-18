import { Module } from '@nestjs/common';
import { CacheModule } from '@nestjs/cache-manager';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ContractModule } from '../contract/contract.module';
import { CreatorEvent } from '../matches/entities/creator-event.entity';
import { CreatorEventLeaderboardEntry } from '../matches/entities/creator-event-leaderboard-entry.entity';
import { CreatorEventPayout } from '../matches/entities/creator-event-payout.entity';
import {
  AdminCreatorEventsController,
  CreatorEventsController,
  PublicCreatorEventsController,
} from './creator-events.controller';
import { CreatorEventsService } from './creator-events.service';

@Module({
  imports: [
    ContractModule,
    TypeOrmModule.forFeature([
      CreatorEvent,
      CreatorEventLeaderboardEntry,
      CreatorEventPayout,
    ]),
    CacheModule.register(),
  ],
  controllers: [
    CreatorEventsController,
    PublicCreatorEventsController,
    AdminCreatorEventsController,
  ],
  providers: [CreatorEventsService],
  exports: [CreatorEventsService],
})
export class CreatorEventsModule {}
