import { Module } from '@nestjs/common';
import { CacheModule } from '@nestjs/cache-manager';
import { TypeOrmModule } from '@nestjs/typeorm';
import { ContractModule } from '../contract/contract.module';
import { CreatorEvent } from '../matches/entities/creator-event.entity';
import { Match } from '../matches/entities/match.entity';
import { MatchPrediction } from '../matches/entities/match-prediction.entity';
import { User } from '../users/entities/user.entity';
import { LeaderboardEntry } from '../leaderboard/entities/leaderboard-entry.entity';
import {
  AdminCreatorEventsController,
  CreatorEventsController,
  PublicCreatorEventsController,
} from './creator-events.controller';
import { CreatorEventsService } from './creator-events.service';

@Module({
  imports: [
    ContractModule,
    TypeOrmModule.forFeature([CreatorEvent, Match, MatchPrediction, User, LeaderboardEntry]),
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
